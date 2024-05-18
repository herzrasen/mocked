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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::HeaderMap;

    use crate::request::Request;
    use crate::routing::matcher::{Matcher, PathParamMatcher};
    use crate::routing::matchers::Matchers;
    use crate::routing::value::Value;

    #[test]
    fn test_or_is_true_when_one_is_true() {
        let m = create_or_matchers();
        let mut path_params = HashMap::new();
        path_params.insert("a".to_string(), "0".to_string());
        path_params.insert("b".to_string(), "2".to_string());
        let req = Request {
            path_params,
            query: HashMap::new(),
            headers: HeaderMap::new(),
            body: "".to_string(),
        };
        let matches = m.matches(&req);
        assert!(matches)
    }

    #[test]
    fn test_or_is_false_when_both_are_false() {
        let m = create_or_matchers();
        let mut path_params = HashMap::new();
        path_params.insert("a".to_string(), "0".to_string());
        let req = Request {
            path_params,
            query: HashMap::new(),
            headers: HeaderMap::new(),
            body: "".to_string(),
        };
        let matches = m.matches(&req);
        assert!(!matches)
    }

    #[test]
    fn test_and_is_true_when_both_are_true() {
        let m = create_and_matchers();
        let mut path_params = HashMap::new();
        path_params.insert("a".to_string(), "1".to_string());
        path_params.insert("b".to_string(), "2".to_string());
        let req = Request {
            path_params,
            query: HashMap::new(),
            headers: HeaderMap::new(),
            body: "".to_string(),
        };
        let matches = m.matches(&req);
        assert!(matches)
    }

    #[test]
    fn test_and_is_false_when_only_is_true() {
        let m = create_and_matchers();
        let mut path_params = HashMap::new();
        path_params.insert("a".to_string(), "1".to_string());
        path_params.insert("b".to_string(), "0".to_string());
        let req = Request {
            path_params,
            query: HashMap::new(),
            headers: HeaderMap::new(),
            body: "".to_string(),
        };
        let matches = m.matches(&req);
        assert!(!matches)
    }

    fn create_or_matchers() -> Matchers {
        Matchers::Or(vec![
            Matcher::PathParam(PathParamMatcher {
                name: "a".to_string(),
                values: vec![Value::Numeric(1.)],
            }),
            Matcher::PathParam(PathParamMatcher {
                name: "b".to_string(),
                values: vec![Value::Numeric(2.)],
            }),
        ])
    }

    fn create_and_matchers() -> Matchers {
        Matchers::And(vec![
            Matcher::PathParam(PathParamMatcher {
                name: "a".to_string(),
                values: vec![Value::Numeric(1.)],
            }),
            Matcher::PathParam(PathParamMatcher {
                name: "b".to_string(),
                values: vec![Value::Numeric(2.)],
            }),
        ])
    }
}
