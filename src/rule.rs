use std::sync::Arc;

use axum::{Extension, Router};
use axum::extract::Request;
use axum::http::StatusCode;
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

    async fn handler(Extension(rule): Extension<Arc<Rule>>, request: Request) -> impl IntoResponse {
        println!("{:?}", request);
        println!("{:?}", rule);
        StatusCode::OK
    }
}

#[cfg(test)]
mod tests {
    use crate::condition::{Condition, PathParamMatch};
    use crate::method::Method;
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
                        condition: Some(Condition::PathParamMatcher(PathParamMatch {
                            name: "bar".to_string(),
                            matches: vec!["foo".to_string(), "baz".to_string()],
                        })),
                        status: 200,
                        body: Some("Hello, World".to_string()),
                        content_type: Some("application/json".to_string()),
                    },
                    Response {
                        condition: None,
                        status: 500,
                        body: None,
                        content_type: None,
                    },
                ],
            }],
        };
        let yaml = serde_yaml::to_string(&rules).unwrap();
        println!("{yaml}");
        // let json = serde_json::to_string_pretty(&rules).unwrap();
        // println!("{json}");
    }
}
