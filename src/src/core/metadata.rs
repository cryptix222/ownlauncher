use crate::core::config::{Game, LaunchMethod};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GameMetadata {
    pub description: String,
    pub banner_url: Option<String>,
}

pub struct MetadataFetcher {
    client: reqwest::Client,
}

impl MetadataFetcher {
    pub fn new() -> Arc<Self> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_default();

        Arc::new(Self { client })
    }

    pub async fn fetch(&self, game: &Game) -> GameMetadata {
        let clean_name = game
            .name
            .split(" [")
            .next()
            .unwrap_or(&game.name)
            .trim()
            .to_string();

        let mut desc = String::new();
        let mut banner = None;

        // 1. Direct Steam AppID
        if let LaunchMethod::Uri { uri } = &game.launch_method {
            if uri.starts_with("steam://rungameid/") {
                let appid = uri.trim_start_matches("steam://rungameid/").trim();
                let hero_url = format!(
                    "https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/library_hero.jpg",
                    appid
                );
                let header_url = format!(
                    "https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg",
                    appid
                );

                if let Ok(head_resp) = self.client.head(&hero_url).send().await {
                    if head_resp.status().is_success() {
                        banner = Some(hero_url);
                    }
                }

                let details_url = format!(
                    "https://store.steampowered.com/api/appdetails?appids={}&l=english",
                    appid
                );
                if let Ok(resp) = self.client.get(&details_url).send().await {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(app_data) = json.get(appid) {
                            if app_data.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                                if let Some(data) = app_data.get("data") {
                                    if let Some(d) = data.get("short_description").and_then(|v| v.as_str()) {
                                        desc = d
                                            .replace("<br>", "\n")
                                            .replace("&quot;", "\"")
                                            .replace("&amp;", "&")
                                            .to_string();
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

        // 2. Steam Search Fallback
        if banner.is_none() || desc.is_empty() {
            let search_url = format!(
                "https://store.steampowered.com/api/storesearch/?term={}&l=english&cc=US",
                url_encode(&clean_name)
            );
            if let Ok(resp) = self.client.get(&search_url).send().await {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
                        if let Some(first_item) = items.first() {
                            if let Some(id) = first_item.get("id").and_then(|v| v.as_i64()) {
                                let hero_url = format!(
                                    "https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/library_hero.jpg",
                                    id
                                );
                                let header_url = format!(
                                    "https://shared.fastly.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg",
                                    id
                                );

                                if banner.is_none() {
                                    if let Ok(head_resp) = self.client.head(&hero_url).send().await {
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
                                    let id_str = id.to_string();
                                    let details_url = format!(
                                        "https://store.steampowered.com/api/appdetails?appids={}&l=english",
                                        id_str
                                    );
                                    if let Ok(d_resp) = self.client.get(&details_url).send().await {
                                        if let Ok(d_json) = d_resp.json::<serde_json::Value>().await {
                                            if let Some(app_data) = d_json.get(&id_str) {
                                                if let Some(data) = app_data.get("data") {
                                                    if let Some(d) = data.get("short_description").and_then(|v| v.as_str()) {
                                                        desc = d
                                                            .replace("<br>", "\n")
                                                            .replace("&quot;", "\"")
                                                            .replace("&amp;", "&")
                                                            .to_string();
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

        // 3. Wikipedia API Fallback
        if desc.is_empty() || banner.is_none() {
            let summary_url = format!(
                "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
                url_encode(&clean_name.replace(' ', "_"))
            );
            if let Ok(resp) = self.client.get(&summary_url).send().await {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
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
        }

        // 4. DuckDuckGo Instant Answer Fallback
        if banner.is_none() || desc.is_empty() {
            let ddg_url = format!(
                "https://api.duckduckgo.com/?q={}+video+game&format=json&no_html=1",
                url_encode(&clean_name)
            );
            if let Ok(resp) = self.client.get(&ddg_url).send().await {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if banner.is_none() {
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

        GameMetadata {
            description: if desc.is_empty() {
                "No description available.".to_string()
            } else {
                desc
            },
            banner_url: banner,
        }
    }

    fn extract_valid_image(val: &serde_json::Value) -> Option<String> {
        if let Some(orig) = val.get("originalimage").or_else(|| val.get("original")) {
            if let Some(src) = orig.get("source").and_then(|v| v.as_str()) {
                let lower = src.to_lowercase();
                if !lower.ends_with(".svg") && !lower.contains(".svg/") {
                    return Some(if src.starts_with("//") {
                        format!("https:{}", src)
                    } else {
                        src.to_string()
                    });
                }
            }
        }
        if let Some(thumb) = val.get("thumbnail") {
            if let Some(src) = thumb.get("source").and_then(|v| v.as_str()) {
                return Some(if src.starts_with("//") {
                    format!("https:{}", src)
                } else {
                    src.to_string()
                });
            }
        }
        None
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
