// This is HUGELY truncated Sōzu code

use serde_derive::{Deserialize, Serialize};
use serde_json;

pub const PROTOCOL_VERSION: u8 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandRequest {
    pub id: String,
    pub version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<u32>,
}

impl CommandRequest {
    pub fn new<T>(id: T, worker_id: Option<u32>) -> CommandRequest
    where
        T: ToString,
    {
        CommandRequest {
            version: PROTOCOL_VERSION,
            id: id.to_string(),
            worker_id,
        }
    }

    pub fn to_serialized_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
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
    pub fn new<T>(id: T, status: CommandStatus, message: T) -> CommandResponse
    where
        T: ToString,
    {
        CommandResponse {
            version: PROTOCOL_VERSION,
            id: id.to_string(),
            status,
            message: message.to_string(),
        }
    }

    pub fn to_serialized_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serde_to_and_from_string_works() {
        let request = CommandRequest::new("some-request-id", Some(23));
        let stringified_request = request.to_serialized_string().unwrap();

        assert_eq!(
            serde_json::from_str::<CommandRequest>(&stringified_request).unwrap(),
            request
        );
    }

    #[test]
    fn deserialize_error_works() {
        let bad_request = "{\"username\":345,\"password\":\"HeyPatric\"}";
        assert!(serde_json::from_str::<CommandRequest>(&bad_request).is_err())
    }
}
