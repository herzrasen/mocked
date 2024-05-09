use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::matcher::Matcher;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Matchers {
    And(Vec<Matcher>),
    Or(Vec<Matcher>),
}

impl Matchers {
    pub fn matches(&self, req: &Request) -> bool {
        match self {
            Matchers::And(matchers) => matchers.iter().all(|matcher| matcher.matches(req)),
            Matchers::Or(matchers) => matchers.iter().any(|matcher| matcher.matches(req)),
        }
    }
}
