use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
pub enum LaunchMethod {
    Exe { path: String },
    Uri { uri: String },
    Client { client_path: String, args: Vec<String> },
    Bnet { client_path: String, code: String },
}

impl LaunchMethod {
    #[allow(dead_code)]
    pub fn platform_name(&self) -> &'static str {
        match self {
            LaunchMethod::Exe { .. } => "Manual",
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub launch_method: LaunchMethod,
    #[serde(default)]
    pub banner_url: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub installed_path: Option<String>,
    #[serde(default)]
    pub last_launched: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub minimize_on_launch: bool,
    #[serde(default)]
    pub close_on_launch: bool,
    #[serde(default = "default_true")]
    pub show_platform_badges: bool,
    #[serde(default)]
    pub compact_sidebar: bool,
    #[serde(default = "default_true")]
    pub auto_scan_on_startup: bool,
    #[serde(default)]
    pub games: Vec<Game>,
}

fn default_theme() -> String {
    "dark".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            minimize_on_launch: false,
            close_on_launch: false,
            show_platform_badges: true,
            compact_sidebar: false,
            auto_scan_on_startup: true,
            games: Vec::new(),
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    if let Some(app_data) = std::env::var_os("APPDATA") {
        PathBuf::from(app_data).join("OwnLauncher")
    } else {
        PathBuf::from(".").join("config")
    }
}

pub fn get_config_file() -> PathBuf {
    get_config_dir().join("games.toml")
}

pub async fn load_config() -> Config {
    let path = get_config_file();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path).await {
            if let Ok(cfg) = toml::from_str::<Config>(&content) {
                return cfg;
            }
        }
    }
    Config::default()
}

pub async fn save_config(config: &Config) -> Result<(), String> {
    let dir = get_config_dir();
    fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create config dir: {}", e))?;

    let toml_data = toml::to_string_pretty(config)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let file_path = get_config_file();
    let temp_path = dir.join("games.toml.tmp");

    fs::write(&temp_path, toml_data)
        .await
        .map_err(|e| format!("Failed to write config: {}", e))?;

    fs::rename(&temp_path, &file_path)
        .await
        .map_err(|e| format!("Failed to commit config rename: {}", e))?;

    Ok(())
}
