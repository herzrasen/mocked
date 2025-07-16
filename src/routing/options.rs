use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {
    pub address: String,
    pub port: u16,
    pub enable_cors: Option<bool>,
    pub min_response_delay_ms: Option<u64>,
    pub max_response_delay_ms: Option<u64>,
}
