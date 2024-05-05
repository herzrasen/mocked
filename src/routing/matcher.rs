use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::Matching;
use crate::routing::r#match::Match;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "of", content = "with")]
pub enum Matcher {
    PathParamMatcher(PathParamMatch),
    HeaderValueContainsMatcher(HeaderValueContainsMatch),
}

impl Matcher {
    pub fn matches(&self, req: &Request) -> bool {
        match self {
            Matcher::PathParamMatcher(matcher) => matcher.matches(req),
            Matcher::HeaderValueContainsMatcher(matcher) => matcher.matches(req),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathParamMatch {
    pub name: String,
    pub matches: Vec<Match>,
}

impl Matching for PathParamMatch {
    fn matches(&self, req: &Request) -> bool {
        if let Some(value) = req.path_params.get(&self.name) {
            let m: Match = value.clone().into();
            let matches = self.matches.contains(&m);
            if matches {
                log::info!("PathParamMatcher matches {value} for {}", self.name);
            }
            return matches;
        }
        false
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderValueContainsMatch {
    pub name: String,
    pub matches: Vec<String>,
}

impl Matching for HeaderValueContainsMatch {
    fn matches(&self, req: &Request) -> bool {
        if let Some(value) = req.headers.get(&self.name) {
            let value_str = value.to_str().unwrap_or("");
            let matches = self.matches.iter().any(|v| value_str.contains(v));
            if matches {
                log::info!(
                    "HeaderValueContainsMatcher matches {value_str} for {}",
                    self.name
                );
            }
            return matches;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::HeaderMap;

    use crate::request::Request;
    use crate::routing::matcher::Matcher::PathParamMatcher;
    use crate::routing::matcher::PathParamMatch;
    use crate::routing::r#match::Match;

    #[test]
    fn test_path_param_matches() {
        let ppm = PathParamMatcher(PathParamMatch {
            name: "param1".to_string(),
            matches: vec![Match::String("valueX".to_string())],
        });
        let mut path_params = HashMap::new();
        path_params.insert("param1".to_string(), "valueX".to_string());
        let req = Request {
            headers: HeaderMap::new(),
            query: HashMap::new(),
            body: "".to_string(),
            path_params,
        };
        let matches = ppm.matches(&req);
        assert!(matches)
    }
}
