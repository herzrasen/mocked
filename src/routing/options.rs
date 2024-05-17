use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Options {
    pub min_response_delay_ms: Option<u64>,
    pub max_response_delay_ms: Option<u64>,
}
