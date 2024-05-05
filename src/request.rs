use std::collections::HashMap;

use axum::http::HeaderMap;

#[derive(Clone, Debug)]
pub struct Request {
    pub headers: HeaderMap,
    pub path_params: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: String,
}
