use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Router};
use axum::extract::{Query, RawPathParams};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{delete, get, head, MethodRouter, patch, post, put};
use serde::{Deserialize, Serialize};

use crate::method::Method;
use crate::response::Response;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rule {
    pub path: String,
    pub method: Method,
    pub responses: Vec<Response>,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub headers: HeaderMap,
    pub path_params: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: String,
}

impl Rule {
    pub fn router(self: Arc<Self>) -> Router {
        Router::new()
            .route(self.path.as_str(), self.clone().handler_for_method())
            .layer(Extension(self))
    }

    fn handler_for_method(self: Arc<Self>) -> MethodRouter {
        match self.method {
            Method::GET => get(Self::handler),
            Method::POST => post(Self::handler),
            Method::PUT => put(Self::handler),
            Method::PATCH => patch(Self::handler),
            Method::DELETE => delete(Self::handler),
            Method::HEAD => head(Self::handler),
        }
    }

    async fn handler(
        Extension(rule): Extension<Arc<Rule>>,
        headers: HeaderMap,
        path_params: RawPathParams,
        Query(query): Query<HashMap<String, String>>,
        body: String,
    ) -> impl IntoResponse {
        let path_params = path_params
            .iter()
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.insert(key.to_string(), value.to_string());
                acc
            });
        let request = Request {
            headers,
            path_params,
            query,
            body,
        };
        if let Some(resp) = rule.clone().select_response(&request) {
            log::info!("Selected response {:?}", resp);
            resp.response()
        } else {
            log::warn!("Unable to select response");
            (StatusCode::NOT_FOUND, "Unable to select response for input").into_response()
        }
    }

    fn select_response(self: Arc<Rule>, req: &Request) -> Option<Response> {
        self.responses.iter().cloned().find(|r| r.matches(&req))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::matcher::{Matcher, PathParamMatch};
    use crate::matchers::Matchers;
    use crate::method::Method;
    use crate::r#match::Match;
    use crate::response::Response;
    use crate::rule::Rule;
    use crate::rules::Rules;

    #[test]
    fn test_rule_to_yaml() {
        let rules = Rules {
            rules: vec![Rule {
                path: "/v1/foo/:bar".to_string(),
                method: Method::GET,
                responses: vec![
                    Response {
                        matchers: Matchers::Or(vec![Matcher::PathParamMatcher(PathParamMatch {
                            name: "bar".to_string(),
                            matches: vec![
                                Match::String("foo".to_string()),
                                Match::String("baz".to_string()),
                            ],
                        })]),
                        status: 200,
                        body: Some("Hello, World".to_string()),
                        headers: HashMap::new(),
                    },
                    Response {
                        matchers: Matchers::empty(),
                        status: 500,
                        body: None,
                        headers: HashMap::new(),
                    },
                ],
            }],
        };
        let yaml = serde_yaml::to_string(&rules).unwrap();
        println!("{yaml}");
    }
}
