use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::Matching;
use crate::routing::value::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "with")]
pub enum Matcher {
    PathParam(PathParamMatcher),
    HeaderContains(HeaderContainsMatcher),
    QueryContains(QueryContainsMatcher)
}

impl Matcher {
    pub fn matches(&self, req: &Request) -> bool {
        match self {
            Matcher::PathParam(matcher) => matcher.matches(req),
            Matcher::HeaderContains(matcher) => matcher.matches(req),
            Matcher::QueryContains(matcher) => matcher.matches(req),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathParamMatcher {
    pub name: String,
    pub values: Vec<Value>,
}

impl Matching for PathParamMatcher {
    fn matches(&self, req: &Request) -> bool {
        if let Some(value) = req.path_params.get(&self.name) {
            let v: Value = value.clone().into();
            let matches = self.values.contains(&v);
            if matches {
                log::info!("PathParamMatcher matches {value} for {}", self.name);
            }
            return matches;
        }
        false
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeaderContainsMatcher {
    pub name: String,
    pub values: Vec<String>,
}

impl Matching for HeaderContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        if let Some(value) = req.headers.get(&self.name) {
            let value_str: &str = value.to_str().unwrap_or("");
            let matches = self.values.iter().any(|v| value_str.contains(v));
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryContainsMatcher {
    pub name: String,
    pub values: Vec<String>,
}

impl Matching for QueryContainsMatcher {
    fn matches(&self, req: &Request) -> bool {
        if let Some(value) = req.query.get(&self.name) {
            let matches = self.values.iter().any(|v| value.contains(v));
            if matches {
                log::info!("QueryContainsMatcher matches {value} for {}", self.name);
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
    use crate::routing::matcher::Matcher::PathParam;
    use crate::routing::matcher::Matcher::QueryContains;
    use crate::routing::matcher::PathParamMatcher;
    use crate::routing::value::Value;

    use super::QueryContainsMatcher;

    #[test]
    fn test_path_param_matches() {
        let ppm = PathParam(PathParamMatcher {
            name: "param1".to_string(),
            values: vec![Value::String("valueX".to_string())],
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

    #[test]
    fn test_query_param_matches() {
        let qcm = QueryContains(QueryContainsMatcher {
            name: "search".to_string(),
            values: vec!["my-value".to_string()]
        });
        let mut query = HashMap::new();
        query.insert("search".to_string(), "my-value".to_string());
        let req: Request = Request {
            headers: HeaderMap::new(),
            query,
            body: "".to_string(),
            path_params: HashMap::new(),
        };
        let matches = qcm.matches(&req);
        assert!(matches)
    }
}
