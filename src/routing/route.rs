use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Query, RawPathParams};
use axum::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN
};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::routing::MethodRouter;
use axum::{Extension, Router};
use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::condition::Condition;
use crate::routing::method::Method;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Route {
    pub path: String,
    pub methods: Vec<Method>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cors: Option<bool>,
    pub conditions: Vec<Condition>,
}

impl Route {
    pub fn router(self: Arc<Self>) -> Router {
        Router::new()
            .route(self.path.as_str(), self.clone().handler_for_methods())
            .layer(Extension(self))
    }

    fn handler_for_methods(self: Arc<Self>) -> MethodRouter {
        let router = self
            .methods
            .iter()
            .fold(MethodRouter::new(), |acc, method| {
                log::info!("Adding {:?} @ {}", method, self.path);
                match method {
                    Method::Get => acc.get(Self::handler),
                    Method::Post => acc.post(Self::handler),
                    Method::Put => acc.put(Self::handler),
                    Method::Patch => acc.patch(Self::handler),
                    Method::Delete => acc.delete(Self::handler),
                    Method::Head => acc.head(Self::handler),
                    Method::Options => acc.options(Self::handler),
                }
            });
        if self.enable_cors.unwrap_or(false) {
            log::info!("Enabling CORS @ {}", self.path);
            router.clone().options(Self::cors_handler)
        } else {
            router
        }
    }

    async fn cors_handler(Extension(route): Extension<Arc<Route>>) -> impl IntoResponse {
        let mut resp = (StatusCode::OK, Body::empty()).into_response();
        let cors_headers = route.clone().cors_headers();
        cors_headers.iter().for_each(|(k, v)| {
            resp.headers_mut().append(k, v.clone());
        });
        resp
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
            let mut resp = condition.response.response();
            if route.enable_cors.unwrap_or(false) {
                log::info!("Adding CORS headers");
                let cors_headers = route.clone().cors_headers();
                cors_headers.iter().for_each(|(k, v)| {
                    resp.headers_mut().append(k, v.clone());
                });
            }
            resp
        } else {
            log::warn!("Unable to select response");
            (StatusCode::NOT_FOUND, "Unable to select response for input").into_response()
        }
    }

    fn cors_headers(self: Arc<Route>) -> HeaderMap<HeaderValue> {
        let methods = self
            .methods
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(",");
        let mut headers = HeaderMap::new();
        headers.append(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
        headers.append(ACCESS_CONTROL_ALLOW_METHODS, methods.parse().unwrap());
        headers.append(ACCESS_CONTROL_ALLOW_HEADERS, "Authorization".parse().unwrap());
        headers.append(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
        headers
    }

    fn select_condition(self: Arc<Route>, req: &Request) -> Option<Condition> {
        self.conditions.iter().cloned().find(|r| r.matches(&req))
    }
}
