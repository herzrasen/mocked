use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub enable_cors: Option<bool>,
    pub min_response_delay_ms: Option<u64>,
    pub max_response_delay_ms: Option<u64>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            address: None,
            port: None,
            enable_cors: Some(false),
            min_response_delay_ms: None,
            max_response_delay_ms: None,
        }
    }
}
