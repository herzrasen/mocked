use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Options {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub min_response_delay_ms: Option<u64>,
    pub max_response_delay_ms: Option<u64>,
}
