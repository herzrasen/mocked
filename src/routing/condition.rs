use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::matcher::Matcher;
use crate::routing::matchers::Matchers;
use crate::routing::response::Response;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    #[serde(flatten)]
    pub matcher: Option<Matcher>,
    #[serde(flatten)]
    pub matchers: Option<Matchers>,
    pub response: Response,
}

impl Condition {
    pub fn matches(&self, req: &Request) -> bool {
        match self.matcher.clone() {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::HeaderMap;

    use crate::request::Request;
    use crate::routing::body::Body;
    use crate::routing::condition::Condition;
    use crate::routing::matcher::{Matcher, PathParamMatcher};
    use crate::routing::matchers::Matchers;
    use crate::routing::response::Response;
    use crate::routing::value::Value;

    #[test]
    fn test_single_matcher_is_evaluated_first() {
        let condition = Condition {
            matcher: Some(Matcher::PathParam(PathParamMatcher {
                name: String::from("foo"),
                values: vec![Value::Numeric(123.)],
            })),
            matchers: Some(Matchers::Or(vec![Matcher::PathParam(PathParamMatcher {
                name: String::from("foo"),
                values: vec![Value::Numeric(234.)],
            })])),
            response: Response {
                status: 200,
                headers: HashMap::new(),
                body: Some(Body::empty()),
            },
        };
        let mut path_params = HashMap::new();
        path_params.insert(String::from("foo"), String::from("234"));
        let mut req = Request {
            path_params: path_params.clone(),
            headers: HeaderMap::new(),
            query: HashMap::new(),
            body: String::new(),
        };
        let matches = condition.matches(&req);
        assert!(!matches);
        path_params.insert(String::from("foo"), String::from("123"));
        req.path_params = path_params;
        let matches = condition.matches(&req);
        assert!(matches)
    }

    #[test]
    fn test_matcher_is_evaluated() {
        let condition = Condition {
            matcher: None,
            matchers: Some(Matchers::Or(vec![Matcher::PathParam(PathParamMatcher {
                name: String::from("foo"),
                values: vec![Value::Numeric(234.)],
            })])),
            response: Response {
                status: 200,
                headers: HashMap::new(),
                body: Some(Body::empty()),
            },
        };
        let mut path_params = HashMap::new();
        path_params.insert(String::from("foo"), String::from("234"));
        let req = Request {
            path_params: path_params.clone(),
            headers: HeaderMap::new(),
            query: HashMap::new(),
            body: String::new(),
        };
        let matches = condition.matches(&req);
        assert!(matches);
    }

    #[test]
    fn test_evaluates_to_true_if_no_matcher_is_defined() {
        let condition = Condition {
            matcher: None,
            matchers: None,
            response: Response {
                status: 200,
                headers: HashMap::new(),
                body: Some(Body::empty()),
            },
        };
        let mut path_params = HashMap::new();
        path_params.insert(String::from("foo"), String::from("234"));
        let req = Request {
            path_params: path_params.clone(),
            headers: HeaderMap::new(),
            query: HashMap::new(),
            body: String::new(),
        };
        let matches = condition.matches(&req);
        assert!(matches);
    }
}
