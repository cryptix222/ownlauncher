#![windows_subsystem = "windows"]

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
enum AppTheme {
    Dark,
    Light,
    Cyberpunk,
    Nord,
    Dracula,
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::Dark
    }
}

impl AppTheme {
    fn name(&self) -> &'static str {
        match self {
            AppTheme::Dark => "Darkmode",
            AppTheme::Light => "Lightmode",
            AppTheme::Cyberpunk => "Cyberpunk",
            AppTheme::Nord => "North",
            AppTheme::Dracula => "Dracula",
        }
    }

    fn apply(&self, ctx: &egui::Context) {
        let mut visuals = match self {
            AppTheme::Dark => {
                let mut v = egui::Visuals::dark();
                v.window_fill = egui::Color32::from_rgb(15, 17, 21);
                v.panel_fill = egui::Color32::from_rgb(15, 17, 21);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(22, 25, 31);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 34, 42);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 52, 64);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(60, 70, 85);
                v.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(220, 225, 235);
                v.selection.bg_fill = egui::Color32::from_rgb(88, 166, 255);
                v
            }
            AppTheme::Light => {
                let mut v = egui::Visuals::light();
                v.window_fill = egui::Color32::from_rgb(245, 247, 250);
                v.panel_fill = egui::Color32::from_rgb(245, 247, 250);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(235, 238, 242);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(220, 225, 232);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(205, 212, 222);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(185, 195, 208);
                v.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(30, 35, 42);
                v.selection.bg_fill = egui::Color32::from_rgb(0, 102, 204);
                v
            }
            AppTheme::Cyberpunk => {
                let mut v = egui::Visuals::dark();
                v.window_fill = egui::Color32::from_rgb(13, 2, 33);
                v.panel_fill = egui::Color32::from_rgb(13, 2, 33);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(25, 10, 50);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 15, 80);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(255, 0, 127);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(0, 240, 255);
                v.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(255, 240, 0);
                v.selection.bg_fill = egui::Color32::from_rgb(255, 0, 127);
                v
            }
            AppTheme::Nord => {
                let mut v = egui::Visuals::dark();
                v.window_fill = egui::Color32::from_rgb(46, 52, 64);
                v.panel_fill = egui::Color32::from_rgb(46, 52, 64);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(59, 66, 82);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(67, 76, 94);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(76, 86, 106);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(136, 192, 208);
                v.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(236, 239, 244);
                v.selection.bg_fill = egui::Color32::from_rgb(136, 192, 208);
                v
            }
            AppTheme::Dracula => {
                let mut v = egui::Visuals::dark();
                v.window_fill = egui::Color32::from_rgb(40, 42, 54);
                v.panel_fill = egui::Color32::from_rgb(40, 42, 54);
                v.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(68, 71, 90);
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(68, 71, 90);
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(98, 114, 164);
                v.widgets.active.bg_fill = egui::Color32::from_rgb(255, 121, 198);
                v.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(248, 248, 242);
                v.selection.bg_fill = egui::Color32::from_rgb(189, 147, 249);
                v
            }
        };

        visuals.window_rounding = egui::Rounding::same(8.0);
        visuals.image_loading_spinners = false;
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.interact_size.y = 26.0;
        ctx.set_style(style);
    }

    fn accent_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Dark => egui::Color32::from_rgb(88, 166, 255),
            AppTheme::Light => egui::Color32::from_rgb(0, 102, 204),
            AppTheme::Cyberpunk => egui::Color32::from_rgb(255, 0, 127),
            AppTheme::Nord => egui::Color32::from_rgb(136, 192, 208),
            AppTheme::Dracula => egui::Color32::from_rgb(189, 147, 249),
        }
    }

    fn card_fill(&self) -> egui::Color32 {
        match self {
            AppTheme::Dark => egui::Color32::from_rgb(22, 25, 31),
            AppTheme::Light => egui::Color32::from_rgb(235, 238, 242),
            AppTheme::Cyberpunk => egui::Color32::from_rgb(25, 10, 50),
            AppTheme::Nord => egui::Color32::from_rgb(59, 66, 82),
            AppTheme::Dracula => egui::Color32::from_rgb(50, 52, 67),
        }
    }

    fn card_border(&self) -> egui::Color32 {
        match self {
            AppTheme::Dark => egui::Color32::from_rgb(45, 50, 60),
            AppTheme::Light => egui::Color32::from_rgb(210, 215, 222),
            AppTheme::Cyberpunk => egui::Color32::from_rgb(0, 240, 255),
            AppTheme::Nord => egui::Color32::from_rgb(76, 86, 106),
            AppTheme::Dracula => egui::Color32::from_rgb(98, 114, 164),
        }
    }

    fn sidebar_item_bg(&self, selected: bool, hovered: bool) -> egui::Color32 {
        match (self, selected, hovered) {
            (AppTheme::Light, true, _) => egui::Color32::from_rgb(215, 225, 240),
            (AppTheme::Light, false, true) => egui::Color32::from_rgb(230, 234, 240),
            (AppTheme::Light, false, false) => egui::Color32::TRANSPARENT,
            (_, true, _) => egui::Color32::from_white_alpha(30),
            (_, false, true) => egui::Color32::from_white_alpha(15),
            (_, false, false) => egui::Color32::TRANSPARENT,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum LaunchMethod {
    Exe { path: String },
    Uri { uri: String },
    Client { client_path: String, args: Vec<String> },
    Bnet { client_path: String, code: String }, 
}

impl LaunchMethod {
    fn platform_name(&self) -> &'static str {
        match self {
            LaunchMethod::Exe { .. } => "Standard / Manual",
            LaunchMethod::Uri { uri } => {
                if uri.starts_with("steam://") {
                    "Steam"
                } else if uri.starts_with("com.epicgames.launcher://") {
                    "Epic Games"
                } else {
                    "URI Protocol"
                }
            }
            LaunchMethod::Client { .. } => "Riot Games",
            LaunchMethod::Bnet { .. } => "Battle.net",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LauncherPlatform {
    Steam,
    Epic,
    Riot,
    Bnet,
    Manual,
}

impl LauncherPlatform {
    fn from_launch_method(method: &LaunchMethod) -> Self {
        match method {
            LaunchMethod::Uri { uri } => {
                if uri.starts_with("steam://") {
                    LauncherPlatform::Steam
                } else if uri.starts_with("com.epicgames.launcher://") {
                    LauncherPlatform::Epic
                } else {
                    LauncherPlatform::Manual
                }
            }
            LaunchMethod::Client { .. } => LauncherPlatform::Riot,
            LaunchMethod::Bnet { .. } => LauncherPlatform::Bnet,
            LaunchMethod::Exe { .. } => LauncherPlatform::Manual,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            LauncherPlatform::Steam => "Steam",
            LauncherPlatform::Epic => "Epic Games",
            LauncherPlatform::Riot => "Riot Games",
            LauncherPlatform::Bnet => "Battle.net",
            LauncherPlatform::Manual => "Standard / Manual",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Game {
    name: String,
    launch_method: LaunchMethod,
}

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    theme: AppTheme,
    #[serde(default)]
    minimize_on_launch: bool,
    #[serde(default)]
    close_on_launch: bool,
    #[serde(default = "default_true")]
    show_platform_badges: bool,
    #[serde(default)]
    compact_sidebar: bool,
    #[serde(default)]
    games: Vec<Game>,
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: AppTheme::Dark,
            minimize_on_launch: false,
            close_on_launch: false,
            show_platform_badges: true,
            compact_sidebar: false,
            games: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct EpicManifest {
    display_name: Option<String>,
    app_name: Option<String>,
}

#[derive(Clone, Default)]
struct GameMetadata {
    description: String,
    banner_url: Option<String>,
    banner_bytes: Option<Arc<[u8]>>,
}

struct LauncherApp {
    config: Config,
    search_query: String,
    new_name: String,
    new_path: String,
    selected_game: Option<usize>,
    settings_open: bool,
    rescan_status: Option<String>,
    
    meta_receiver: mpsc::Receiver<GameMetadata>,
    meta_sender: mpsc::Sender<GameMetadata>,
    current_meta: Option<GameMetadata>,
    is_fetching: bool,
}

fn get_config_path() -> PathBuf {
    let mut path = if let Some(app_data) = std::env::var_os("APPDATA") {
        PathBuf::from(app_data)
    } else {
        PathBuf::from(".")
    };
    path.push("OwnLauncher");
    let _ = fs::create_dir_all(&path);
    path.push("games.toml");
    path
}

impl LauncherApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let config_path = get_config_path();
        let mut config = if config_path.exists() {
        let data = fs::read_to_string(&config_path).unwrap_or_default();
            toml::from_str(&data).unwrap_or_default()
        } else {
            Config::default()
        };

        config.theme.apply(&cc.egui_ctx);

        Self::scan_actual_games(&mut config);
        config.games.sort_by(|a, b| a.name.cmp(&b.name));
        

        let (meta_sender, meta_receiver) = mpsc::channel();

        Self {
            config,
            search_query: String::new(),
            new_name: String::new(),
            new_path: String::new(),
            selected_game: None,
            settings_open: false,
            rescan_status: None,
            meta_receiver,
            meta_sender,
            current_meta: None,
            is_fetching: false,
        }
    }

    fn scan_actual_games(config: &mut Config) {
        let drives = ["C", "D", "E", "F", "G", "H"];
        
        let mut steam_paths = vec![
            PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps"),
            PathBuf::from("C:\\Program Files\\Steam\\steamapps"),
        ];
        
        for drive in drives {
            steam_paths.push(PathBuf::from(format!("{}:\\Steam\\steamapps", drive)));
            steam_paths.push(PathBuf::from(format!("{}:\\SteamLibrary\\steamapps", drive)));
            steam_paths.push(PathBuf::from(format!("{}:\\Spiele\\Steam\\steamapps", drive)));
            steam_paths.push(PathBuf::from(format!("{}:\\Games\\Steam\\steamapps", drive)));
        }

        for base_path in steam_paths {
            if base_path.exists() {
                if let Ok(entries) = fs::read_dir(&base_path) {
                    for entry in entries.flatten() {
                        let filename = entry.file_name().to_string_lossy().to_string();
                        if filename.starts_with("appmanifest_") && filename.ends_with(".acf") {
                            let appid = filename.replace("appmanifest_", "").replace(".acf", "");
                            if let Ok(content) = fs::read_to_string(entry.path()) {
                                let mut name = String::new();
                                for line in content.lines() {
                                    if line.contains("\"name\"") {
                                        let parts: Vec<&str> = line.split('"').collect();
                                        if parts.len() >= 4 { name = parts[3].to_string(); break; }
                                    }
                                }
                                if !name.is_empty() {
                                    Self::add_game(config, format!("{} [Steam]", name), LaunchMethod::Uri { uri: format!("steam://rungameid/{}", appid) });
                                }
                            }
                        }
                    }
                }
            }
        }

        let epic_manifest_dir = PathBuf::from("C:\\ProgramData\\Epic\\EpicGamesLauncher\\Data\\Manifests");
        if epic_manifest_dir.exists() {
            if let Ok(entries) = fs::read_dir(epic_manifest_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("item") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(manifest) = serde_json::from_str::<EpicManifest>(&content) {
                                if let (Some(name), Some(app_name)) = (manifest.display_name, manifest.app_name) {
                                    Self::add_game(config, format!("{} [Epic]", name), LaunchMethod::Uri { uri: format!("com.epicgames.launcher://apps/{}?action=launch&silent=true", app_name) });
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut bnet_client = None;
        let mut bnet_client_paths = vec![];
        for drive in drives {
            bnet_client_paths.push(format!("{}:\\Program Files (x86)\\Battle.net\\Battle.net.exe", drive));
            bnet_client_paths.push(format!("{}:\\Program Files\\Battle.net\\Battle.net.exe", drive));
            bnet_client_paths.push(format!("{}:\\Battle.net\\Battle.net.exe", drive));
        }
        
        for path in bnet_client_paths { 
            if Path::new(&path).exists() { 
                bnet_client = Some(path.to_string()); 
                break; 
            } 
        }

        if let Some(client_path) = bnet_client {
            let bnet_games = vec![
                ("Overwatch", "Pro", "Overwatch"), 
                ("World of Warcraft", "WoW", "World of Warcraft"), 
                ("Diablo IV", "Fen", "Diablo IV"), 
                ("Call of Duty", "Odin", "Call of Duty")
            ];
            
            let mut bnet_base_paths = vec![];
            for drive in drives {
                bnet_base_paths.push(format!("{}:\\Program Files (x86)", drive));
                bnet_base_paths.push(format!("{}:\\Program Files", drive));
                bnet_base_paths.push(format!("{}:\\Games", drive));
                bnet_base_paths.push(format!("{}:\\Spiele", drive));
            }
            
            for (name, code, folder) in bnet_games {
                for base in &bnet_base_paths {
                    if PathBuf::from(base).join(folder).exists() {
                        Self::add_game(config, format!("{} [Battle.net]", name), LaunchMethod::Bnet { client_path: client_path.clone(), code: code.to_string() });
                        break;
                    }
                }
            }
        }

        let mut riot_client = None;
        for drive in drives {
            let p = PathBuf::from(format!("{}:\\Riot Games\\Riot Client\\RiotClientServices.exe", drive));
            if p.exists() {
                riot_client = Some(p);
                break;
            }
        }

        if let Some(riot_client_path) = riot_client {
            let riot_games = vec![
                ("VALORANT", "valorant", "live"), 
                ("League of Legends", "league_of_legends", "live")
            ];
            for (display_name, product_code, patchline) in riot_games {
                for drive in drives {
                    if PathBuf::from(format!("{}:\\Riot Games\\{}", drive, display_name)).exists() {
                        Self::add_game(config, format!("{} [Riot]", display_name), LaunchMethod::Client { 
                            client_path: riot_client_path.to_string_lossy().into_owned(), 
                            args: vec![format!("--launch-product={}", product_code), format!("--launch-patchline={}", patchline)] 
                        });
                        break;
                    }
                }
            }
        }
        
        let _ = fs::write(get_config_path(), toml::to_string(config).unwrap_or_default());
    }

    fn add_game(config: &mut Config, name: String, method: LaunchMethod) {
        if !config.games.iter().any(|g| g.name == name) { config.games.push(Game { name, launch_method: method }); }
    }

    fn save_config(&self) {
    if let Ok(toml_string) = toml::to_string(&self.config) { 
        let _ = fs::write(get_config_path(), toml_string); 
    }
}

    fn extract_valid_image(val: &serde_json::Value) -> Option<String> {
        if let Some(orig) = val.get("originalimage").or_else(|| val.get("original")) {
            if let Some(src) = orig.get("source").and_then(|v| v.as_str()) {
                let lower = src.to_lowercase();
                if !lower.ends_with(".svg") && !lower.contains(".svg/") {
                    return Some(if src.starts_with("//") { format!("https:{}", src) } else { src.to_string() });
                }
            }
        }
        if let Some(thumb) = val.get("thumbnail") {
            if let Some(src) = thumb.get("source").and_then(|v| v.as_str()) {
                return Some(if src.starts_with("//") { format!("https:{}", src) } else { src.to_string() });
            }
        }
        None
    }

    fn fetch_metadata_background(game: &Game) -> GameMetadata {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(6))
            .build()
            .unwrap_or_default();

        let mut desc = String::new();
        let mut banner = None;

        let clean_name = game.name.split(" [").next().unwrap_or(&game.name).trim().to_string();

        // 1. Direct Steam AppID if launched via Steam URI
        if let LaunchMethod::Uri { uri } = &game.launch_method {
            if uri.starts_with("steam://rungameid/") {
                let appid = uri.replace("steam://rungameid/", "").trim().to_string();
                let hero_url = format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/library_hero.jpg", appid);
                let header_url = format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg", appid);

                if let Ok(head_resp) = client.head(&hero_url).send() {
                    if head_resp.status().is_success() {
                        banner = Some(hero_url);
                    }
                }

                let url = format!("https://store.steampowered.com/api/appdetails?appids={}&l=english", appid);
                if let Ok(resp) = client.get(&url).send() {
                    if let Ok(json) = resp.json::<serde_json::Value>() {
                        if let Some(app_data) = json.get(&appid) {
                            if app_data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                                if let Some(data) = app_data.get("data") {
                                    if let Some(d) = data.get("short_description").and_then(|v| v.as_str()) {
                                        desc = d.replace("<br>", "\n").replace("&quot;", "\"").replace("&amp;", "&").to_string();
                                    }
                                    if banner.is_none() {
                                        if let Some(h) = data.get("header_image").and_then(|v| v.as_str()) {
                                            banner = Some(h.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if banner.is_none() {
                    banner = Some(header_url);
                }
            }
        }

        // 2. Automatic Steam Store Search fallback
        if banner.is_none() || desc.is_empty() {
            let search_url = format!(
                "https://store.steampowered.com/api/storesearch/?term={}&l=english&cc=US",
                url_encode(&clean_name)
            );
            if let Ok(resp) = client.get(&search_url).send() {
                if let Ok(json) = resp.json::<serde_json::Value>() {
                    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
                        if let Some(first_item) = items.first() {
                            if let Some(id) = first_item.get("id").and_then(|v| v.as_i64()) {
                                let hero_url = format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/library_hero.jpg", id);
                                let header_url = format!("https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg", id);

                                if banner.is_none() {
                                    if let Ok(head_resp) = client.head(&hero_url).send() {
                                        if head_resp.status().is_success() {
                                            banner = Some(hero_url);
                                        } else {
                                            banner = Some(header_url.clone());
                                        }
                                    } else {
                                        banner = Some(header_url.clone());
                                    }
                                }

                                if desc.is_empty() {
                                    let details_url = format!("https://store.steampowered.com/api/appdetails?appids={}&l=english", id);
                                    if let Ok(d_resp) = client.get(&details_url).send() {
                                        if let Ok(d_json) = d_resp.json::<serde_json::Value>() {
                                            let id_str = id.to_string();
                                            if let Some(app_data) = d_json.get(&id_str) {
                                                if app_data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                                                    if let Some(data) = app_data.get("data") {
                                                        if let Some(d) = data.get("short_description").and_then(|v| v.as_str()) {
                                                            desc = d.replace("<br>", "\n").replace("&quot;", "\"").replace("&amp;", "&").to_string();
                                                        }
                                                        if banner.is_none() {
                                                            if let Some(h) = data.get("header_image").and_then(|v| v.as_str()) {
                                                                banner = Some(h.to_string());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Automatic Wikipedia Fallback (for non-Steam games)
        if desc.is_empty() || banner.is_none() {
            let summary_url = format!(
                "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
                url_encode(&clean_name.replace(' ', "_"))
            );
            if let Ok(resp) = client.get(&summary_url).send() {
                if let Ok(json) = resp.json::<serde_json::Value>() {
                    if desc.is_empty() {
                        if let Some(d) = json.get("extract").and_then(|v| v.as_str()) {
                            if !d.is_empty() {
                                desc = d.to_string();
                            }
                        }
                    }
                    if banner.is_none() {
                        banner = Self::extract_valid_image(&json);
                    }
                }
            }

            if desc.is_empty() || banner.is_none() {
                let search_queries = [
                    clean_name.clone(),
                    format!("{} video game", clean_name),
                ];

                for query in &search_queries {
                    let search_url = format!(
                        "https://en.wikipedia.org/w/api.php?action=query&format=json&generator=search&gsrsearch={}&gsrlimit=3&prop=pageimages|extracts&piprop=original|thumbnail&pithumbsize=1200&exintro=1&explaintext=1",
                        url_encode(query)
                    );
                    if let Ok(resp) = client.get(&search_url).send() {
                        if let Ok(json) = resp.json::<serde_json::Value>() {
                            if let Some(pages) = json.get("query").and_then(|q| q.get("pages")).and_then(|p| p.as_object()) {
                                for (_, page) in pages {
                                    if desc.is_empty() {
                                        if let Some(extract) = page.get("extract").and_then(|v| v.as_str()) {
                                            if !extract.is_empty() {
                                                desc = extract.to_string();
                                            }
                                        }
                                    }
                                    if banner.is_none() {
                                        banner = Self::extract_valid_image(page);
                                    }
                                    if !desc.is_empty() && banner.is_some() {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if !desc.is_empty() && banner.is_some() {
                        break;
                    }
                }
            }
        }

        // 4. DuckDuckGo Instant Answer Fallback
        if banner.is_none() {
            let ddg_url = format!("https://api.duckduckgo.com/?q={}+video+game&format=json&no_html=1", url_encode(&clean_name));
            if let Ok(resp) = client.get(&ddg_url).send() {
                if let Ok(json) = resp.json::<serde_json::Value>() {
                    if let Some(img) = json.get("Image").and_then(|v| v.as_str()) {
                        if !img.is_empty() && !img.to_lowercase().ends_with(".svg") {
                            let full_img = if img.starts_with("//") {
                                format!("https:{}", img)
                            } else if img.starts_with('/') {
                                format!("https://duckduckgo.com{}", img)
                            } else {
                                img.to_string()
                            };
                            banner = Some(full_img);
                        }
                    }
                    if desc.is_empty() {
                        if let Some(abstract_text) = json.get("AbstractText").and_then(|v| v.as_str()) {
                            if !abstract_text.is_empty() {
                                desc = abstract_text.to_string();
                            }
                        }
                    }
                }
            }
        }

        let mut banner_bytes = None;
        if let Some(ref url) = banner {
            if let Ok(resp) = client.get(url).send() {
                if resp.status().is_success() {
                    if let Ok(bytes) = resp.bytes() {
                        if !bytes.is_empty() {
                            banner_bytes = Some(Arc::from(bytes.as_ref()));
                        }
                    }
                }
            }
        }

        GameMetadata {
            description: if desc.is_empty() { "Keine Beschreibung verfügbar.".to_string() } else { desc },
            banner_url: banner,
            banner_bytes,
        }
    }

    fn select_game_index(&mut self, idx: usize, ctx: &egui::Context) {
        if self.selected_game != Some(idx) {
            self.selected_game = Some(idx);
            self.current_meta = None; 
            self.is_fetching = true;  

            let sender = self.meta_sender.clone();
            let game_clone = self.config.games[idx].clone();
            let ctx_clone = ctx.clone();
            std::thread::spawn(move || {
                let meta = Self::fetch_metadata_background(&game_clone);
                let _ = sender.send(meta);
                ctx_clone.request_repaint(); 
            });
        }
    }

    fn launch_game(method: LaunchMethod) {
        std::thread::spawn(move || {
            match method {
                LaunchMethod::Exe { path } => {
                    let p = Path::new(&path);
                    let parent = p.parent().unwrap_or(Path::new(""));
                    let _ = Command::new(&path).current_dir(parent).spawn();
                }
                LaunchMethod::Uri { uri } => { let _ = open::that(&uri); }
                LaunchMethod::Client { client_path, args } => {
                    let p = Path::new(&client_path);
                    let parent = p.parent().unwrap_or(Path::new(""));
                    let _ = Command::new(&client_path).args(&args).current_dir(parent).spawn();
                }
                LaunchMethod::Bnet { client_path, code } => {
                    let p = Path::new(&client_path);
                    let parent = p.parent().unwrap_or(Path::new(""));
                    #[cfg(target_os = "windows")]
                    {
                        use std::os::windows::process::CommandExt;
                        let raw_command = format!("--exec=\"launch {}\"", code);
                        let is_running = if let Ok(output) = Command::new("tasklist").output() {
                            String::from_utf8_lossy(&output.stdout).to_lowercase().contains("battle.net.exe")
                        } else { false };

                        if is_running {
                            let _ = Command::new(&client_path).current_dir(parent).raw_arg(&raw_command).spawn();
                        } else {
                            let _ = Command::new(&client_path).current_dir(parent).spawn();
                            let client_clone = client_path.clone();
                            let parent_clone = parent.to_path_buf();
                            std::thread::spawn(move || {
                                std::thread::sleep(std::time::Duration::from_secs(15)); 
                                let _ = Command::new(&client_clone).current_dir(parent_clone).raw_arg(&raw_command).spawn();
                            });
                        }
                    }
                }
            }
        });
    }

    fn draw_platform_logo(painter: &egui::Painter, rect: egui::Rect, platform: LauncherPlatform) {
        let center = rect.center();
        let radius = (rect.width().min(rect.height()) / 2.0).min(10.0);

        match platform {
            LauncherPlatform::Steam => {
                painter.circle_filled(center, radius, egui::Color32::from_rgb(23, 36, 50));
                painter.circle_stroke(center, radius, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(42, 71, 94)));

                let cyan = egui::Color32::from_rgb(102, 192, 244);
                let p1 = egui::pos2(center.x - 2.8, center.y + 2.8);
                let p2 = egui::pos2(center.x + 2.8, center.y - 2.8);

                painter.line_segment([p1, p2], egui::Stroke::new(2.0_f32, cyan));
                painter.circle_filled(p2, 3.2, cyan);
                painter.circle_filled(p1, 1.8, cyan);
                painter.circle_filled(p1, 0.8, egui::Color32::from_rgb(23, 36, 50));
            }
            LauncherPlatform::Epic => {
                painter.circle_filled(center, radius, egui::Color32::from_rgb(20, 20, 20));
                painter.circle_stroke(center, radius, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(70, 70, 70)));
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    "E",
                    egui::FontId::proportional(11.0),
                    egui::Color32::WHITE,
                );
            }
            LauncherPlatform::Bnet => {
                painter.circle_filled(center, radius, egui::Color32::from_rgb(0, 30, 65));
                painter.circle_stroke(center, radius, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(0, 116, 224)));

                let blue = egui::Color32::from_rgb(0, 174, 239);
                let n1 = egui::pos2(center.x, center.y - 4.0);
                let n2 = egui::pos2(center.x + 3.6, center.y + 2.4);
                let n3 = egui::pos2(center.x - 3.6, center.y + 2.4);

                painter.line_segment([n1, n2], egui::Stroke::new(1.2_f32, blue));
                painter.line_segment([n2, n3], egui::Stroke::new(1.2_f32, blue));
                painter.line_segment([n3, n1], egui::Stroke::new(1.2_f32, blue));
                painter.circle_filled(n1, 1.4, egui::Color32::WHITE);
                painter.circle_filled(n2, 1.4, egui::Color32::WHITE);
                painter.circle_filled(n3, 1.4, egui::Color32::WHITE);
            }
            LauncherPlatform::Riot => {
                painter.circle_filled(center, radius, egui::Color32::from_rgb(215, 0, 35));
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    "R",
                    egui::FontId::proportional(11.0),
                    egui::Color32::WHITE,
                );
            }
            LauncherPlatform::Manual => {
                painter.circle_filled(center, radius, egui::Color32::from_rgb(35, 40, 50));
                painter.circle_stroke(center, radius, egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(70, 75, 85)));
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    "▶",
                    egui::FontId::proportional(9.0),
                    egui::Color32::from_rgb(180, 195, 215),
                );
            }
        }
    }

    fn draw_skeleton(ui: &mut egui::Ui, width: f32, height: f32, rounding: f32, active_theme: AppTheme) {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
        let time = ui.input(|i| i.time);
        let pulse = ((time * 3.5).sin() * 0.5 + 0.5) as f32;
        let base = active_theme.card_fill();
        let shimmer_alpha = (20.0 + pulse * 35.0) as u8;
        let shimmer = egui::Color32::from_white_alpha(shimmer_alpha);

        ui.painter().rect_filled(rect, egui::Rounding::same(rounding), base);
        ui.painter().rect_filled(rect, egui::Rounding::same(rounding), shimmer);
        ui.painter().rect_stroke(
            rect,
            egui::Rounding::same(rounding),
            egui::Stroke::new(1.0_f32, active_theme.card_border().linear_multiply(0.5 + pulse * 0.5)),
        );
    }

    fn render_game_detail(&mut self, ui: &mut egui::Ui, idx: usize, active_theme: AppTheme) {
        let game = self.config.games[idx].clone();
        let clean_title = game.name.split(" [").next().unwrap_or(&game.name).trim();
        let platform = LauncherPlatform::from_launch_method(&game.launch_method);

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if self.is_fetching {
                    ui.ctx().request_repaint();

                    // Skeleton Loader for Banner
                    let full_width = ui.available_width();
                    Self::draw_skeleton(ui, full_width, 220.0, 12.0, active_theme);
                    ui.add_space(15.0);

                    // Title & Buttons
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new(clean_title)
                                    .size(30.0)
                                    .strong()
                            );
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Platform:").size(13.0).weak());
                                let (logo_rect, _) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::hover());
                                Self::draw_platform_logo(ui.painter(), logo_rect, platform);
                                ui.label(egui::RichText::new(platform.name()).size(13.0).strong());
                            });
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let play_btn = egui::Button::new(
                                egui::RichText::new("▶ PLAY")
                                    .size(18.0)
                                    .color(egui::Color32::WHITE)
                                    .strong()
                            )
                            .fill(egui::Color32::from_rgb(35, 165, 89))
                            .min_size(egui::vec2(140.0, 46.0))
                            .rounding(egui::Rounding::same(8.0));

                            if ui.add(play_btn).clicked() {
                                Self::launch_game(game.launch_method.clone());
                                if self.config.close_on_launch {
                                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                                } else if self.config.minimize_on_launch {
                                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                                }
                            }
                        });
                    });

                    ui.add_space(20.0);

                    // Skeleton Cards
                    ui.columns(2, |cols| {
                        egui::Frame::none()
                            .fill(active_theme.card_fill())
                            .stroke(egui::Stroke::new(1.0_f32, active_theme.card_border()))
                            .inner_margin(12.0)
                            .rounding(egui::Rounding::same(8.0))
                            .show(&mut cols[0], |ui| {
                                ui.set_min_height(180.0);
                                Self::draw_skeleton(ui, 80.0, 18.0, 4.0, active_theme);
                                ui.add_space(12.0);
                                let w = ui.available_width();
                                Self::draw_skeleton(ui, w * 0.95, 13.0, 4.0, active_theme);
                                ui.add_space(8.0);
                                Self::draw_skeleton(ui, w * 0.88, 13.0, 4.0, active_theme);
                                ui.add_space(8.0);
                                Self::draw_skeleton(ui, w * 0.72, 13.0, 4.0, active_theme);
                                ui.add_space(8.0);
                                Self::draw_skeleton(ui, w * 0.50, 13.0, 4.0, active_theme);
                            });

                        egui::Frame::none()
                            .fill(active_theme.card_fill())
                            .stroke(egui::Stroke::new(1.0_f32, active_theme.card_border()))
                            .inner_margin(12.0)
                            .rounding(egui::Rounding::same(8.0))
                            .show(&mut cols[1], |ui| {
                                ui.set_min_height(180.0);
                                Self::draw_skeleton(ui, 120.0, 18.0, 4.0, active_theme);
                                ui.add_space(14.0);
                                let w = ui.available_width();
                                Self::draw_skeleton(ui, w * 0.85, 16.0, 4.0, active_theme);
                                ui.add_space(10.0);
                                Self::draw_skeleton(ui, w * 0.60, 16.0, 4.0, active_theme);
                            });
                    });

                    ui.add_space(20.0);
                    return;
                }

                if let Some(meta) = &self.current_meta {
                    if let Some(bytes) = &meta.banner_bytes {
                        let img = egui::Image::from_bytes(format!("bytes://banner_{}", idx), bytes.clone())
                            .rounding(egui::Rounding::same(12.0))
                            .max_height(240.0)
                            .max_width(ui.available_width())
                            .show_loading_spinner(false);
                        ui.add(img);
                        ui.add_space(15.0);
                    } else if let Some(banner) = &meta.banner_url {
                        let img = egui::Image::new(banner)
                            .rounding(egui::Rounding::same(12.0))
                            .max_height(240.0)
                            .max_width(ui.available_width())
                            .show_loading_spinner(false);
                        ui.add(img);
                        ui.add_space(15.0);
                    } else {
                        // Fallback Banner
                        let (banner_rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 130.0), egui::Sense::hover());
                        ui.painter().rect_filled(banner_rect, egui::Rounding::same(12.0), active_theme.card_fill());
                        ui.painter().rect_stroke(banner_rect, egui::Rounding::same(12.0), egui::Stroke::new(1.0_f32, active_theme.card_border()));
                        ui.painter().text(
                            banner_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("🎮 {}", clean_title),
                            egui::FontId::proportional(22.0),
                            active_theme.accent_color(),
                        );
                        ui.add_space(15.0);
                    }
                }

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(clean_title)
                                .size(30.0)
                                .strong()
                        );
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Platform:").size(13.0).weak());
                            let (logo_rect, _) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::hover());
                            Self::draw_platform_logo(ui.painter(), logo_rect, platform);
                            ui.label(egui::RichText::new(platform.name()).size(13.0).strong());
                        });
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let play_btn = egui::Button::new(
                            egui::RichText::new("▶ PLAY")
                                .size(18.0)
                                .color(egui::Color32::WHITE)
                                .strong()
                        )
                        .fill(egui::Color32::from_rgb(35, 165, 89))
                        .min_size(egui::vec2(140.0, 46.0))
                        .rounding(egui::Rounding::same(8.0));

                        if ui.add(play_btn).clicked() {
                            Self::launch_game(game.launch_method.clone());
                            if self.config.close_on_launch {
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            } else if self.config.minimize_on_launch {
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                            }
                        }

                        let del_btn = egui::Button::new(
                            egui::RichText::new("Remove").size(14.0)
                        )
                        .min_size(egui::vec2(85.0, 38.0))
                        .rounding(egui::Rounding::same(6.0));
                        
                        if ui.add(del_btn).clicked() {
                            self.config.games.remove(idx);
                            self.save_config();
                            self.selected_game = None;
                        }
                    });
                });

                ui.add_space(20.0);
                
                ui.columns(2, |cols| {
                    egui::Frame::none()
                        .fill(active_theme.card_fill())
                        .stroke(egui::Stroke::new(1.0_f32, active_theme.card_border()))
                        .inner_margin(12.0)
                        .rounding(egui::Rounding::same(8.0))
                        .show(&mut cols[0], |ui| {
                            ui.set_min_height(180.0);
                            ui.label(egui::RichText::new("About").size(16.0).strong());
                            ui.add_space(6.0);
                            if let Some(meta) = &self.current_meta {
                                ui.add(egui::Label::new(&meta.description).wrap());
                            } else {
                                ui.label("Keine Beschreibung verfügbar.");
                            }
                        });

                    egui::Frame::none()
                        .fill(active_theme.card_fill())
                        .stroke(egui::Stroke::new(1.0_f32, active_theme.card_border()))
                        .inner_margin(12.0)
                        .rounding(egui::Rounding::same(8.0))
                        .show(&mut cols[1], |ui| {
                            ui.set_min_height(180.0);
                            ui.label(egui::RichText::new("System Details").size(16.0).strong());
                            ui.add_space(8.0);
                            
                            egui::Grid::new("game_tech_details")
                                .num_columns(2)
                                .spacing([12.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("Launcher:").strong());
                                    ui.label(game.launch_method.platform_name());
                                    ui.end_row();
                                });
                        });
                });
                
                ui.add_space(20.0);
            });
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        if let Ok(meta) = self.meta_receiver.try_recv() {
            self.current_meta = Some(meta);
            self.is_fetching = false;
        }

        let active_theme = self.config.theme;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("🚀 OwnLauncher").strong());

                ui.add_space(15.0);

                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("🔍 Suche...")
                        .desired_width(180.0)
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let add_game_btn = ui.menu_button("➕ Add Game", |ui| {
                        ui.set_min_width(260.0);
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_name);
                        ui.label("Location (.exe):");
                        ui.text_edit_singleline(&mut self.new_path);
                        ui.add_space(5.0);
                        if ui.button("Save").clicked() && !self.new_name.is_empty() && !self.new_path.is_empty() {
                            self.config.games.push(Game {
                                name: self.new_name.clone(),
                                launch_method: LaunchMethod::Exe { path: self.new_path.clone() },
                            });
                            self.config.games.sort_by(|a, b| a.name.cmp(&b.name));
                            self.save_config();
                            self.new_name.clear();
                            self.new_path.clear();
                            ui.close_menu();
                        }
                    });

                    ui.add_space(8.0);

                    let settings_btn = ui.button("⚙");
                    if settings_btn.clicked() {
                        self.settings_open = !self.settings_open;
                    }

                });
            });
            ui.add_space(10.0);
        });

        let mut newly_selected = None;

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(240.0)
            .width_range(180.0..=350.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let full_width = ui.available_width();
                        let item_height = if self.config.compact_sidebar { 30.0 } else { 38.0 };

                        for (idx, game) in self.config.games.iter().enumerate() {
                            if !self.search_query.is_empty() 
                                && !game.name.to_lowercase().contains(&self.search_query.to_lowercase()) {
                                continue;
                            }

                            let is_selected = self.selected_game == Some(idx);
                            let (rect, response) = ui.allocate_exact_size(egui::vec2(full_width, item_height), egui::Sense::click());

                            if ui.is_rect_visible(rect) {
                                let hover = response.hovered();
                                let bg_color = active_theme.sidebar_item_bg(is_selected, hover);

                                ui.painter().rect_filled(rect, egui::Rounding::same(6.0), bg_color);

                                if is_selected {
                                    let indicator = egui::Rect::from_min_max(
                                        rect.min,
                                        egui::pos2(rect.min.x + 4.0, rect.max.y)
                                    );
                                    ui.painter().rect_filled(indicator, egui::Rounding::same(2.0), active_theme.accent_color());
                                }

                                let text_color = ui.visuals().text_color();

                                let display_name = if self.config.show_platform_badges {
                                    &game.name
                                } else {
                                    game.name.split(" [").next().unwrap_or(&game.name).trim()
                                };

                                ui.painter().text(
                                    egui::pos2(rect.min.x + 14.0, rect.center().y),
                                    egui::Align2::LEFT_CENTER,
                                    display_name,
                                    egui::FontId::proportional(if self.config.compact_sidebar { 13.0 } else { 14.0 }),
                                    text_color,
                                );
                            }

                            if response.clicked() {
                                newly_selected = Some(idx);
                            }
                        }
                    });
            });

        if let Some(idx) = newly_selected {
            self.select_game_index(idx, ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.selected_game {
                if idx < self.config.games.len() {
                    self.render_game_detail(ui, idx, active_theme);
                } else {
                    self.selected_game = None;
                }
            } 
        });

        if self.settings_open {
            let mut open = self.settings_open;
            let mut close_requested = false;
            egui::Window::new("⚙ Settings")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .default_width(420.0)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    ui.heading(egui::RichText::new("🎨 Themes").size(16.0).strong());
                    ui.add_space(4.0);
                    
                    let themes = [
                        AppTheme::Dark,
                        AppTheme::Light,
                        AppTheme::Cyberpunk,
                        AppTheme::Nord,
                        AppTheme::Dracula,
                    ];
                    
                    let mut theme_changed = false;
                    for t in &themes {
                        let is_selected = self.config.theme == *t;
                        ui.horizontal(|ui| {
                            let (dot_rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                            ui.painter().circle_filled(dot_rect.center(), 6.0, t.accent_color());
                            if ui.selectable_label(is_selected, t.name()).clicked() && !is_selected {
                                self.config.theme = *t;
                                theme_changed = true;
                            }
                        });
                    }
                    if theme_changed {
                        self.config.theme.apply(ctx);
                        self.save_config();
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);

                    ui.heading(egui::RichText::new("🚀 Start options").size(16.0).strong());
                    ui.add_space(4.0);
                    if ui.checkbox(&mut self.config.minimize_on_launch, "Minimize OwnLauncher on game start").changed() {
                        if self.config.minimize_on_launch { self.config.close_on_launch = false; }
                        self.save_config();
                    }
                    if ui.checkbox(&mut self.config.close_on_launch, "Close OwnLauncher after game start").changed() {
                        if self.config.close_on_launch { self.config.minimize_on_launch = false; }
                        self.save_config();
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);

                    ui.heading(egui::RichText::new("🎮 Sidebar").size(16.0).strong());
                    ui.add_space(4.0);
                    if ui.checkbox(&mut self.config.show_platform_badges, "Show Platform ([Steam], [Epic] etc.)").changed() {
                        self.save_config();
                    }
                    if ui.checkbox(&mut self.config.compact_sidebar, "Compact View").changed() {
                        self.save_config();
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);

                    ui.heading(egui::RichText::new("🔄 Game Libary").size(16.0).strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        if ui.button("🔄 Scan Games new").clicked() {
                            Self::scan_actual_games(&mut self.config);
                            self.config.games.sort_by(|a, b| a.name.cmp(&b.name));
                            self.save_config();
                            self.rescan_status = Some(format!("Scan Complete: {} Game(s) found", self.config.games.len()));
                        }

                        if ui.button("Load descriptions new").clicked() {
                            if let Some(idx) = self.selected_game {
                                self.current_meta = None;
                                self.is_fetching = true;
                                let sender = self.meta_sender.clone();
                                let game_clone = self.config.games[idx].clone();
                                let ctx_clone = ctx.clone();
                                std::thread::spawn(move || {
                                    let meta = Self::fetch_metadata_background(&game_clone);
                                    let _ = sender.send(meta);
                                    ctx_clone.request_repaint();
                                });
                            }
                        }
                    });

                    if let Some(status) = &self.rescan_status {
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new(status).size(12.0).color(active_theme.accent_color()));
                    }

                    
                });
            if close_requested {
                open = false;
            }
            self.settings_open = open;
        }
    }
}

fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                use std::fmt::Write;
                let _ = write!(encoded, "%{:02X}", b);
            }
        }
    }
    encoded
}

fn load_icon() -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon_bytes = include_bytes!("../assets/ownicon.ico");
        let image = image::load_from_memory(icon_bytes)
            .expect("Fehler beim Laden von assets/ownicon.ico")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 680.0])
            .with_min_inner_size([780.0, 500.0])
            .with_icon(Arc::new(load_icon())),
        ..Default::default()
    };
    
    eframe::run_native(
        "OwnLauncher",
        options,
        Box::new(|cc| Ok(Box::new(LauncherApp::new(cc)))),
    )
}

