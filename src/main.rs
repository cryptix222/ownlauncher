#![windows_subsystem = "windows"]

mod bridge;
mod core;

use bridge::dispatcher::Dispatcher;
use bridge::protocol::BridgeRequest;
use core::config;
use core::metadata::MetadataFetcher;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::sync::Arc;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use tokio::sync::RwLock;
use wry::http::Response;
use wry::WebViewBuilder;

#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct Asset;

#[derive(Debug)]
pub enum UserEvent {
    DispatchScript(String),
    Minimize,
    ToggleMaximize,
    Close,
    DragWindow,
}

fn get_mime_type(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "application/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "ttf" => "font/ttf",
        _ => "application/octet-stream",
    }
}

fn load_icon() -> Option<tao::window::Icon> {
    let bytes = include_bytes!("../assets/ownicon.ico");
    let img = image::load_from_memory(bytes).ok()?.into_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    tao::window::Icon::from_rgba(rgba, width, height).ok()
}

fn main() {
    // 1. Initialize Tokio multi-thread runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio runtime");

    // 2. Load configuration asynchronously
    let initial_config = runtime.block_on(async { config::load_config().await });
    let config_state = Arc::new(RwLock::new(initial_config));
    let metadata_fetcher = MetadataFetcher::new();

    // 3. Setup Tao Event Loop
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let mut window_builder = WindowBuilder::new()
        .with_title("OwnLauncher")
        .with_inner_size(LogicalSize::new(1120.0, 720.0))
        .with_min_inner_size(LogicalSize::new(860.0, 560.0))
        .with_decorations(false) // Frameless window for native Windows 11 style titlebar
        .with_resizable(true);

    if let Some(icon) = load_icon() {
        window_builder = window_builder.with_window_icon(Some(icon));
    }

    let window = window_builder
        .build(&event_loop)
        .expect("Failed to create native desktop window");

    // 4. Create Dispatcher
    let dispatcher = Dispatcher::new(
        config_state.clone(),
        metadata_fetcher.clone(),
        proxy.clone(),
    );

    let tokio_handle = runtime.handle().clone();

    // IPC Initialization JS bootstrap
    let init_script = r#"
        window.__LAUNCHER_PENDING__ = new Map();
        window.__LAUNCHER_LISTENERS__ = new Map();

        window.__LAUNCHER_RESPONSE__ = function(resp) {
            const resolver = window.__LAUNCHER_PENDING__.get(resp.id);
            if (resolver) {
                window.__LAUNCHER_PENDING__.delete(resp.id);
                if (resp.success) {
                    resolver.resolve(resp.data);
                } else {
                    resolver.reject(new Error(resp.error || 'Bridge error'));
                }
            }
        };

        window.__LAUNCHER_EVENT__ = function(event) {
            const list = window.__LAUNCHER_LISTENERS__.get(event.type) || [];
            for (const cb of list) {
                try { cb(event.payload); } catch(err) { console.error('Error in event handler:', err); }
            }
        };
    "#;

    // 5. Build WebView with embedded assets (100% standalone binary)
    let dispatcher_clone = dispatcher.clone();
    let webview = WebViewBuilder::new()
        .with_devtools(cfg!(debug_assertions))
        .with_initialization_script(init_script)
        .with_ipc_handler(move |req| {
            let body = req.body().clone();
            let disp = dispatcher_clone.clone();
            tokio_handle.spawn(async move {
                if let Ok(parsed) = serde_json::from_str::<BridgeRequest>(&body) {
                    disp.dispatch(parsed).await;
                }
            });
        })
        .with_custom_protocol("app".into(), move |_id, req| {
            let raw_path = req.uri().path();
            let clean_path = raw_path.trim_start_matches('/');
            let target_path = if clean_path.is_empty() || clean_path == "index.html" {
                "index.html"
            } else {
                clean_path
            };

            // 1. Direct asset lookup in binary memory
            if let Some(file) = Asset::get(target_path) {
                let mime = get_mime_type(target_path);
                Response::builder()
                    .header("Content-Type", mime)
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Cache-Control", "public, max-age=31536000")
                    .body(file.data)
                    .unwrap()
            } else if let Some(index_file) = Asset::get("index.html") {
                // 2. SPA routing fallback to index.html
                Response::builder()
                    .header("Content-Type", "text/html; charset=utf-8")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(index_file.data)
                    .unwrap()
            } else {
                // 3. Fallback error response
                Response::builder()
                    .status(404)
                    .header("Content-Type", "text/plain")
                    .body(Cow::Borrowed(&b"Asset not found in executable binary"[..]))
                    .unwrap()
            }
        })
        .with_url("app://localhost")
        .build(&window)
        .expect("Failed to initialize WebView");

    // 6. Native Window Event Loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::DispatchScript(script)) => {
                let _ = webview.evaluate_script(&script);
            }
            Event::UserEvent(UserEvent::Minimize) => {
                window.set_minimized(true);
            }
            Event::UserEvent(UserEvent::ToggleMaximize) => {
                let is_max = window.is_maximized();
                window.set_maximized(!is_max);
                let script = format!(
                    "window.__ON_WINDOW_MAXIMIZE_CHANGED && window.__ON_WINDOW_MAXIMIZE_CHANGED(!{});",
                    is_max
                );
                let _ = webview.evaluate_script(&script);
            }
            Event::UserEvent(UserEvent::Close) => {
                *control_flow = ControlFlow::Exit;
                std::process::exit(0);
            }
            Event::UserEvent(UserEvent::DragWindow) => {
                let _ = window.drag_window();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                let is_max = window.is_maximized();
                let script = format!(
                    "window.__ON_WINDOW_MAXIMIZE_CHANGED && window.__ON_WINDOW_MAXIMIZE_CHANGED({});",
                    is_max
                );
                let _ = webview.evaluate_script(&script);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                std::process::exit(0);
            }
            _ => {}
        }
    });
}
