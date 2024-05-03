use std::sync::Arc;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, head, patch, post, put, MethodRouter};
use axum::{Extension, Router};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

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

#[cfg(test)]
mod tests {
    use crate::rule::{Condition, Method, PathParamMatch, Response, Rule};
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
