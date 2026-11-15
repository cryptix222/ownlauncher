use crate::core::config::{Config, Game};
use crate::core::metadata::GameMetadata;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct BridgeRequest {
    pub id: String,
    pub action: String,
    #[serde(default)]
    pub payload: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct BridgeResponse {
    pub id: String,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum BridgeEvent {
    ConfigLoaded(Config),
    GamesDiscovered(Vec<Game>),
    MetadataLoaded {
        game_id: String,
        metadata: GameMetadata,
    },
    Log {
        level: String,
        message: String,
        timestamp: u64,
    },
    Status {
        text: String,
        is_busy: bool,
    },
}
