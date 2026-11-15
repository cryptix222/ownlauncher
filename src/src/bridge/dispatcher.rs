use crate::bridge::protocol::{BridgeEvent, BridgeRequest, BridgeResponse};
use crate::core::config::{self, Config, Game, LaunchMethod};
use crate::core::launcher::GameLauncher;
use crate::core::metadata::MetadataFetcher;
use crate::core::scanner::Scanner;
use crate::UserEvent;
use std::sync::Arc;
use tao::event_loop::EventLoopProxy;
use tokio::sync::RwLock;

pub struct Dispatcher {
    config: Arc<RwLock<Config>>,
    fetcher: Arc<MetadataFetcher>,
    proxy: EventLoopProxy<UserEvent>,
}

impl Dispatcher {
    pub fn new(
        config: Arc<RwLock<Config>>,
        fetcher: Arc<MetadataFetcher>,
        proxy: EventLoopProxy<UserEvent>,
    ) -> Arc<Self> {
        Arc::new(Self {
            config,
            fetcher,
            proxy,
        })
    }

    pub fn send_response(&self, id: String, success: bool, data: Option<serde_json::Value>, error: Option<String>) {
        let resp = BridgeResponse {
            id,
            success,
            data,
            error,
        };
        if let Ok(json) = serde_json::to_string(&resp) {
            let script = format!("window.__LAUNCHER_RESPONSE__({})", json);
            let _ = self.proxy.send_event(UserEvent::DispatchScript(script));
        }
    }

    pub fn emit_event(&self, event: BridgeEvent) {
        if let Ok(json) = serde_json::to_string(&event) {
            let script = format!("window.__LAUNCHER_EVENT__({})", json);
            let _ = self.proxy.send_event(UserEvent::DispatchScript(script));
        }
    }

    pub fn log(&self, level: &str, message: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.emit_event(BridgeEvent::Log {
            level: level.to_string(),
            message: message.to_string(),
            timestamp,
        });
    }

    pub async fn dispatch(self: Arc<Self>, req: BridgeRequest) {
        let action = req.action.as_str();
        let id = req.id.clone();

        match action {
            "get_config" => {
                let cfg = self.config.read().await.clone();
                let val = serde_json::to_value(cfg).unwrap_or(serde_json::Value::Null);
                self.send_response(id, true, Some(val), None);
            }

            "save_config" => {
                if let Ok(new_cfg) = serde_json::from_value::<Config>(req.payload) {
                    if let Err(e) = config::save_config(&new_cfg).await {
                        self.send_response(id, false, None, Some(e));
                    } else {
                        *self.config.write().await = new_cfg.clone();
                        self.emit_event(BridgeEvent::ConfigLoaded(new_cfg.clone()));
                        self.send_response(id, true, Some(serde_json::to_value(new_cfg).unwrap()), None);
                        self.log("info", "Configuration saved successfully.");
                    }
                } else {
                    self.send_response(id, false, None, Some("Invalid configuration format".to_string()));
                }
            }

            "scan_games" => {
                self.log("info", "Starting scan for installed games...");
                self.emit_event(BridgeEvent::Status {
                    text: "Scanning game library...".to_string(),
                    is_busy: true,
                });

                let scanned = Scanner::scan_all_games().await;
                let count = scanned.len();

                {
                    let mut cfg = self.config.write().await;
                    for sg in scanned {
                        if let Some(existing) = cfg.games.iter_mut().find(|g| g.id == sg.id) {
                            existing.launch_method = sg.launch_method;
                            if existing.banner_url.is_none() {
                                existing.banner_url = sg.banner_url;
                            }
                            if existing.installed_path.is_none() {
                                existing.installed_path = sg.installed_path;
                            }
                        } else {
                            cfg.games.push(sg);
                        }
                    }
                    cfg.games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                    let _ = config::save_config(&cfg).await;
                    self.emit_event(BridgeEvent::GamesDiscovered(cfg.games.clone()));
                    self.send_response(id, true, Some(serde_json::to_value(&cfg.games).unwrap()), None);
                }

                self.emit_event(BridgeEvent::Status {
                    text: format!("Scan complete: {} games found", count),
                    is_busy: false,
                });
                self.log("success", &format!("Library scan complete: {} games indexed.", count));

                // Automatically fetch missing descriptions in the background
                let config_bg = self.config.clone();
                let fetcher_bg = self.fetcher.clone();
                let proxy_bg = self.proxy.clone();
                tokio::spawn(async move {
                    let missing: Vec<Game> = {
                        let cfg = config_bg.read().await;
                        cfg.games
                            .iter()
                            .filter(|g| g.description.is_none())
                            .cloned()
                            .collect()
                    };

                    for game in missing {
                        let meta = fetcher_bg.fetch(&game).await;
                        {
                            let mut cfg = config_bg.write().await;
                            if let Some(target) = cfg.games.iter_mut().find(|g| g.id == game.id) {
                                target.description = Some(meta.description.clone());
                                if target.banner_url.is_none() {
                                    target.banner_url = meta.banner_url.clone();
                                }
                            }
                            let _ = config::save_config(&cfg).await;
                        }

                        let evt = BridgeEvent::MetadataLoaded {
                            game_id: game.id,
                            metadata: meta,
                        };
                        if let Ok(json) = serde_json::to_string(&evt) {
                            let script = format!("window.__LAUNCHER_EVENT__({})", json);
                            let _ = proxy_bg.send_event(UserEvent::DispatchScript(script));
                        }

                        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
                    }
                });
            }

            "fetch_metadata" => {
                let game_id = req.payload.get("game_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                if game_id.is_empty() {
                    self.send_response(id, false, None, Some("game_id is required".to_string()));
                    return;
                }

                let game_opt = {
                    let cfg = self.config.read().await;
                    cfg.games.iter().find(|g| g.id == game_id).cloned()
                };

                if let Some(game) = game_opt {
                    self.log("info", &format!("Fetching metadata for: {}", game.name));
                    let meta = self.fetcher.fetch(&game).await;

                    {
                        let mut cfg = self.config.write().await;
                        if let Some(target) = cfg.games.iter_mut().find(|g| g.id == game_id) {
                            if target.banner_url.is_none() || target.banner_url.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
                                target.banner_url = meta.banner_url.clone();
                            }
                            target.description = Some(meta.description.clone());
                        }
                        let _ = config::save_config(&cfg).await;
                    }

                    self.emit_event(BridgeEvent::MetadataLoaded {
                        game_id: game_id.clone(),
                        metadata: meta.clone(),
                    });
                    self.send_response(id, true, Some(serde_json::to_value(&meta).unwrap()), None);
                    self.log("success", &format!("Metadata updated for {}.", game.name));
                } else {
                    self.send_response(id, false, None, Some("Game not found".to_string()));
                }
            }

            "add_game" => {
                let name = req.payload.get("name").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
                let path = req.payload.get("path").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();

                if name.is_empty() || path.is_empty() {
                    self.send_response(id, false, None, Some("Name and path must not be empty".to_string()));
                    return;
                }

                let game_id = format!("manual:{}", url_safe_id(&name));
                let new_game = Game {
                    id: game_id,
                    name: name.clone(),
                    platform: "Manual".to_string(),
                    launch_method: LaunchMethod::Exe { path: path.clone() },
                    banner_url: None,
                    description: None,
                    installed_path: Some(path),
                    last_launched: None,
                };

                {
                    let mut cfg = self.config.write().await;
                    cfg.games.push(new_game.clone());
                    cfg.games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                    let _ = config::save_config(&cfg).await;
                    self.emit_event(BridgeEvent::GamesDiscovered(cfg.games.clone()));
                }

                self.send_response(id, true, Some(serde_json::to_value(&new_game).unwrap()), None);
                self.log("info", &format!("Manual game added: {}", name));
            }

            "remove_game" => {
                let game_id = req.payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let mut removed = false;

                {
                    let mut cfg = self.config.write().await;
                    let initial_len = cfg.games.len();
                    cfg.games.retain(|g| g.id != game_id);
                    if cfg.games.len() < initial_len {
                        removed = true;
                        let _ = config::save_config(&cfg).await;
                        self.emit_event(BridgeEvent::GamesDiscovered(cfg.games.clone()));
                    }
                }

                if removed {
                    self.send_response(id, true, None, None);
                    self.log("info", &format!("Game removed: {}", game_id));
                } else {
                    self.send_response(id, false, None, Some("Game not found in library".to_string()));
                }
            }

            "launch_game" => {
                let game_id = req.payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let (game_opt, minimize, close) = {
                    let cfg = self.config.read().await;
                    (
                        cfg.games.iter().find(|g| g.id == game_id).cloned(),
                        cfg.minimize_on_launch,
                        cfg.close_on_launch,
                    )
                };

                if let Some(game) = game_opt {
                    self.log("info", &format!("Launching: {}...", game.name));
                    match GameLauncher::launch(&game.launch_method) {
                        Ok(_) => {
                            let now = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs();

                            {
                                let mut cfg = self.config.write().await;
                                if let Some(target) = cfg.games.iter_mut().find(|g| g.id == game_id) {
                                    target.last_launched = Some(now);
                                }
                                let _ = config::save_config(&cfg).await;
                            }

                            self.send_response(id, true, None, None);
                            self.log("success", &format!("{} launched successfully.", game.name));

                            if close {
                                let _ = self.proxy.send_event(UserEvent::Close);
                            } else if minimize {
                                let _ = self.proxy.send_event(UserEvent::Minimize);
                            }
                        }
                        Err(e) => {
                            self.send_response(id, false, None, Some(e.clone()));
                            self.log("error", &format!("Error launching {}: {}", game.name, e));
                        }
                    }
                } else {
                    self.send_response(id, false, None, Some("Game not found".to_string()));
                }
            }

            "open_folder" => {
                let path = req.payload.get("path").and_then(|v| v.as_str()).unwrap_or("");
                if path.is_empty() {
                    self.send_response(id, false, None, Some("Path is required".to_string()));
                } else {
                    match GameLauncher::open_folder(path) {
                        Ok(_) => self.send_response(id, true, None, None),
                        Err(e) => self.send_response(id, false, None, Some(e)),
                    }
                }
            }

            "open_url" => {
                let url = req.payload.get("url").and_then(|v| v.as_str()).unwrap_or("");
                if url.is_empty() {
                    self.send_response(id, false, None, Some("URL is required".to_string()));
                } else {
                    let _ = open::that_detached(url);
                    self.send_response(id, true, None, None);
                }
            }

            "window_minimize" => {
                let _ = self.proxy.send_event(UserEvent::Minimize);
                self.send_response(id, true, None, None);
            }

            "window_maximize_toggle" => {
                let _ = self.proxy.send_event(UserEvent::ToggleMaximize);
                self.send_response(id, true, None, None);
            }

            "window_close" => {
                let _ = self.proxy.send_event(UserEvent::Close);
                self.send_response(id, true, None, None);
            }

            "window_drag" => {
                let _ = self.proxy.send_event(UserEvent::DragWindow);
                self.send_response(id, true, None, None);
            }

            unknown => {
                self.send_response(id, false, None, Some(format!("Unknown action: {}", unknown)));
            }
        }
    }
}

fn url_safe_id(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == ' ' || c == '-' || c == '_' {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}
