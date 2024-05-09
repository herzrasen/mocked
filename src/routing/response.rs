use std::collections::HashMap;
use std::io;

use axum::http::{HeaderName, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::routing::body::Body;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub status: u16,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
}

impl Response {
    pub fn response(self) -> axum::response::Response {
        let status_code = StatusCode::from_u16(self.status).unwrap();
        let result: Result<String, io::Error> = self.body.unwrap_or_else(Body::empty).try_into();
        match result {
            Ok(body) => {
                let mut resp = (status_code, body).into_response();
                self.headers.into_iter().for_each(|(header, value)| {
                    let header_name: HeaderName = header.parse().unwrap();
                    resp.headers_mut()
                        .insert(header_name, value.parse().unwrap());
                });
                resp
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error creating response: {:?}", e),
            )
                .into_response(),
        }
    }
}
