use std::collections::HashMap;

use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::matchers::Matchers;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(flatten)]
    pub matchers: Matchers,
    pub status: u16,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl Response {
    pub fn matches(&self, req: &Request) -> bool {
        self.matchers.matches(req)
    }

    pub fn response(self) -> axum::response::Response {
        let status_code = StatusCode::from_u16(self.status).unwrap();

        let mut resp = (status_code, self.body.unwrap_or_else(String::new)).into_response();
        self.headers.into_iter().for_each(|(header, value)| {
            let header_name: HeaderName = header.parse().unwrap();
            resp.headers_mut()
                .insert(header_name, value.parse().unwrap());
        });

        resp
    }
}
