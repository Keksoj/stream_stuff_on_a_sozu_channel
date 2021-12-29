// This is HUGELY truncated Sōzu code

use serde_derive::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandRequest {
    pub id: String,
    pub version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<u32>,
}

impl CommandRequest {
    pub fn new(id: String, worker_id: Option<u32>) -> CommandRequest {
        CommandRequest {
            version: PROTOCOL_VERSION,
            id,
            worker_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandStatus {
    Ok,
    Processing,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandResponse {
    pub id: String,
    pub version: u8,
    pub status: CommandStatus,
    pub message: String,
}

impl CommandResponse {
    pub fn new(id: String, status: CommandStatus, message: String) -> CommandResponse {
        CommandResponse {
            version: PROTOCOL_VERSION,
            id,
            status,
            message,
        }
    }
}
