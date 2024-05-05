use serde::{Deserialize, Serialize};

use crate::r#match::Match;
use crate::rule::Request;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "of", content = "with")]
pub enum Matcher {
    PathParamMatcher(PathParamMatch),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathParamMatch {
    pub name: String,
    pub matches: Vec<Match>,
}

impl Matcher {
    pub fn matches(&self, req: &Request) -> bool {
        match self {
            Matcher::PathParamMatcher(matcher) => {
                if let Some(value) = req.path_params.get(&matcher.name) {
                    let m: Match = value.clone().into();
                    let matches = matcher.matches.contains(&m);
                    if matches {
                        log::info!("PathParamMatcher matches {value} for {}", matcher.name);
                    }
                    return matches;
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::HeaderMap;

    use crate::matcher::Matcher::PathParamMatcher;
    use crate::matcher::PathParamMatch;
    use crate::r#match::Match;
    use crate::rule::Request;

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
