use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "of", content = "with")]
pub enum Condition {
    PathParamMatcher(PathParamMatch),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathParamMatch {
    pub name: String,
    pub matches: Vec<String>,
}
