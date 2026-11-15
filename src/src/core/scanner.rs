use crate::core::config::{Game, LaunchMethod};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct EpicManifest {
    display_name: Option<String>,
    app_name: Option<String>,
    install_location: Option<String>,
}

pub struct Scanner;

impl Scanner {
    pub async fn scan_all_games() -> Vec<Game> {
        let mut games = Vec::new();

        // 1. Steam Scans
        Self::scan_steam(&mut games).await;

        // 2. Epic Games Scans
        Self::scan_epic(&mut games).await;

        // 3. Battle.net Scans
        Self::scan_bnet(&mut games).await;

        // 4. Riot Games Scans
        Self::scan_riot(&mut games).await;

        games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        games
    }

    async fn scan_steam(games: &mut Vec<Game>) {
        let drives = ["C", "D", "E", "F", "G", "H"];
        let mut base_steam_paths = vec![
            PathBuf::from(r"C:\Program Files (x86)\Steam\steamapps"),
            PathBuf::from(r"C:\Program Files\Steam\steamapps"),
        ];

        for drive in drives {
            base_steam_paths.push(PathBuf::from(format!(r"{}:\Steam\steamapps", drive)));
            base_steam_paths.push(PathBuf::from(format!(r"{}:\SteamLibrary\steamapps", drive)));
            base_steam_paths.push(PathBuf::from(format!(r"{}:\Spiele\Steam\steamapps", drive)));
            base_steam_paths.push(PathBuf::from(format!(r"{}:\Games\Steam\steamapps", drive)));
        }

        for folder in base_steam_paths {
            if !folder.exists() {
                continue;
            }

            if let Ok(mut entries) = fs::read_dir(&folder).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let filename = entry.file_name().to_string_lossy().to_string();
                    if filename.starts_with("appmanifest_") && filename.ends_with(".acf") {
                        let appid = filename
                            .trim_start_matches("appmanifest_")
                            .trim_end_matches(".acf")
                            .to_string();

                        if let Ok(content) = fs::read_to_string(entry.path()).await {
                            let mut name = String::new();
                            let mut install_dir = String::new();

                            for line in content.lines() {
                                let trimmed = line.trim();
                                if trimmed.starts_with("\"name\"") {
                                    let parts: Vec<&str> = trimmed.split('"').collect();
                                    if parts.len() >= 4 {
                                        name = parts[3].to_string();
                                    }
                                } else if trimmed.starts_with("\"installdir\"") {
                                    let parts: Vec<&str> = trimmed.split('"').collect();
                                    if parts.len() >= 4 {
                                        install_dir = parts[3].to_string();
                                    }
                                }
                            }

                            if !name.is_empty() {
                                let id = format!("steam:{}", appid);
                                if !games.iter().any(|g| g.id == id) {
                                    let game_path = if !install_dir.is_empty() {
                                        Some(
                                            folder
                                                .join("common")
                                                .join(install_dir)
                                                .to_string_lossy()
                                                .into_owned(),
                                        )
                                    } else {
                                        None
                                    };

                                    games.push(Game {
                                        id,
                                        name,
                                        platform: "Steam".to_string(),
                                        launch_method: LaunchMethod::Uri {
                                            uri: format!("steam://rungameid/{}", appid),
                                        },
                                        banner_url: Some(format!(
                                            "https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/library_hero.jpg",
                                            appid
                                        )),
                                        description: None,
                                        installed_path: game_path,
                                        last_launched: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn scan_epic(games: &mut Vec<Game>) {
        let epic_dir = PathBuf::from(r"C:\ProgramData\Epic\EpicGamesLauncher\Data\Manifests");
        if !epic_dir.exists() {
            return;
        }

        if let Ok(mut entries) = fs::read_dir(epic_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("item") {
                    if let Ok(content) = fs::read_to_string(&path).await {
                        if let Ok(manifest) = serde_json::from_str::<EpicManifest>(&content) {
                            if let (Some(name), Some(app_name)) =
                                (manifest.display_name, manifest.app_name)
                            {
                                let id = format!("epic:{}", app_name);
                                if !games.iter().any(|g| g.id == id) {
                                    games.push(Game {
                                        id,
                                        name,
                                        platform: "Epic Games".to_string(),
                                        launch_method: LaunchMethod::Uri {
                                            uri: format!(
                                                "com.epicgames.launcher://apps/{}?action=launch&silent=true",
                                                app_name
                                            ),
                                        },
                                        banner_url: None,
                                        description: None,
                                        installed_path: manifest.install_location,
                                        last_launched: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn scan_bnet(games: &mut Vec<Game>) {
        let drives = ["C", "D", "E", "F", "G", "H"];
        let mut bnet_client = None;

        for drive in drives {
            let candidates = [
                format!(r"{}:\Program Files (x86)\Battle.net\Battle.net.exe", drive),
                format!(r"{}:\Program Files\Battle.net\Battle.net.exe", drive),
                format!(r"{}:\Battle.net\Battle.net.exe", drive),
            ];
            for c in candidates {
                if Path::new(&c).exists() {
                    bnet_client = Some(c);
                    break;
                }
            }
            if bnet_client.is_some() {
                break;
            }
        }

        if let Some(client_path) = bnet_client {
            let bnet_games = [
                ("Overwatch", "Pro", "Overwatch"),
                ("World of Warcraft", "WoW", "World of Warcraft"),
                ("Diablo IV", "Fen", "Diablo IV"),
                ("Call of Duty", "Odin", "Call of Duty"),
                ("Hearthstone", "WTCG", "Hearthstone"),
                ("StarCraft II", "S2", "StarCraft II"),
            ];

            let mut base_paths = Vec::new();
            for drive in drives {
                base_paths.push(format!(r"{}:\Program Files (x86)", drive));
                base_paths.push(format!(r"{}:\Program Files", drive));
                base_paths.push(format!(r"{}:\Games", drive));
                base_paths.push(format!(r"{}:\Spiele", drive));
            }

            for (display_name, code, folder_name) in bnet_games {
                for base in &base_paths {
                    let folder = PathBuf::from(base).join(folder_name);
                    if folder.exists() {
                        let id = format!("bnet:{}", code);
                        if !games.iter().any(|g| g.id == id) {
                            games.push(Game {
                                id,
                                name: display_name.to_string(),
                                platform: "Battle.net".to_string(),
                                launch_method: LaunchMethod::Bnet {
                                    client_path: client_path.clone(),
                                    code: code.to_string(),
                                },
                                banner_url: None,
                                description: None,
                                installed_path: Some(folder.to_string_lossy().into_owned()),
                                last_launched: None,
                            });
                        }
                        break;
                    }
                }
            }
        }
    }

    async fn scan_riot(games: &mut Vec<Game>) {
        let drives = ["C", "D", "E", "F", "G", "H"];
        let mut riot_client = None;

        for drive in drives {
            let candidate = PathBuf::from(format!(
                r"{}:\Riot Games\Riot Client\RiotClientServices.exe",
                drive
            ));
            if candidate.exists() {
                riot_client = Some(candidate);
                break;
            }
        }

        if let Some(client_path) = riot_client {
            let riot_games = [
                ("VALORANT", "valorant", "live"),
                ("League of Legends", "league_of_legends", "live"),
            ];

            for (display_name, product_code, patchline) in riot_games {
                for drive in drives {
                    let game_folder = PathBuf::from(format!(r"{}:\Riot Games\{}", drive, display_name));
                    if game_folder.exists() {
                        let id = format!("riot:{}", product_code);
                        if !games.iter().any(|g| g.id == id) {
                            games.push(Game {
                                id,
                                name: display_name.to_string(),
                                platform: "Riot Games".to_string(),
                                launch_method: LaunchMethod::Client {
                                    client_path: client_path.to_string_lossy().into_owned(),
                                    args: vec![
                                        format!("--launch-product={}", product_code),
                                        format!("--launch-patchline={}", patchline),
                                    ],
                                },
                                banner_url: None,
                                description: None,
                                installed_path: Some(game_folder.to_string_lossy().into_owned()),
                                last_launched: None,
                            });
                        }
                        break;
                    }
                }
            }
        }
    }
}
