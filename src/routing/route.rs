use std::collections::HashMap;
use std::sync::Arc;

use axum::{Extension, Router};
use axum::extract::{Query, RawPathParams};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::MethodRouter;
use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::condition::Condition;
use crate::routing::method::Method;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Route {
    pub path: String,
    pub methods: Vec<Method>,
    pub conditions: Vec<Condition>,
}

impl Route {
    pub fn router(self: Arc<Self>) -> Router {
        Router::new()
            .route(self.path.as_str(), self.clone().handler_for_methods())
            .layer(Extension(self))
    }

    fn handler_for_methods(self: Arc<Self>) -> MethodRouter {
        self.methods
            .iter()
            .fold(MethodRouter::new(), |acc, method| {
                log::info!("Chaining {:?} for {}", method, self.path);
                match method {
                    Method::Get => acc.get(Self::handler),
                    Method::Post => acc.post(Self::handler),
                    Method::Put => acc.put(Self::handler),
                    Method::Patch => acc.patch(Self::handler),
                    Method::Delete => acc.delete(Self::handler),
                    Method::Head => acc.head(Self::handler),
                    Method::Options => acc.options(Self::handler)
                }
            })
    }

    async fn handler(
        Extension(route): Extension<Arc<Route>>,
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
        if let Some(condition) = route.clone().select_condition(&request) {
            log::info!("Matched condition {:?}", condition);
            condition.response.response()
        } else {
            log::warn!("Unable to select response");
            (StatusCode::NOT_FOUND, "Unable to select response for input").into_response()
        }
    }

    fn select_condition(self: Arc<Route>, req: &Request) -> Option<Condition> {
        self.conditions.iter().cloned().find(|r| r.matches(&req))
    }
}
