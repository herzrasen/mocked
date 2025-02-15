use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub enable_cors: Option<bool>,
    pub min_response_delay_ms: Option<u64>,
    pub max_response_delay_ms: Option<u64>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            host: None,
            port: None,
            enable_cors: Some(false),
            min_response_delay_ms: None,
            max_response_delay_ms: None,
        }
    }
}
