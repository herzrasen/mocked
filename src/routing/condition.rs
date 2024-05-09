use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::matcher::Matcher;
use crate::routing::matchers::Matchers;
use crate::routing::response::Response;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    #[serde(flatten)]
    pub single_matcher: Option<Matcher>,
    #[serde(flatten)]
    pub matchers: Option<Matchers>,
    pub response: Response,
}

impl Condition {
    pub fn matches(&self, req: &Request) -> bool {
        match self.single_matcher.clone() {
            Some(single_matcher) => single_matcher.matches(&req),
            None => match self.matchers.clone() {
                Some(matchers) => matchers.matches(&req),
                None => {
                    // both are unset => request matches in any case
                    true
                }
            },
        }
    }
}
