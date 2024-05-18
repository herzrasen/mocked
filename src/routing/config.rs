use std::sync::Arc;

use axum::Router;
use serde::{Deserialize, Serialize};

use crate::routing::route::Route;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub routes: Vec<Route>,
}

impl Config {
    pub fn router(&self) -> Router {
        let router = self
            .routes
            .iter()
            .cloned()
            .fold(Router::new(), |acc, next| {
                let route = Arc::new(next);
                acc.merge(route.router())
            });
        router
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::http::{HeaderName, HeaderValue};
    use axum_test::TestServer;

    use crate::routing::config::Config;

    #[tokio::test]
    async fn test_create_router() {
        let config_str = r#"routes:
                  - path: /test
                    methods:
                      - POST
                      - PUT
                    conditions:
                      - type: HeaderContains
                        with:
                          name: Authorization
                          values:
                            - Basic
                        response:
                          status: 200
                          headers:
                            Content-Type: test/plain
                          body: >
                            this is a
                            string"#;
        let config: Config = serde_yaml::from_str(config_str).unwrap();
        println!("{:?}", config);
        let router = config.router();
        let server = TestServer::new(router).unwrap();
        let auth_header = HeaderName::from_lowercase(b"authorization").unwrap();
        let header_value = HeaderValue::from_str("Basic foofoo").unwrap();
        let resp = server
            .put("/test")
            .add_header(auth_header, header_value)
            .await;
        assert!(resp.headers().contains_key("Content-Type"));
        resp.assert_status_ok();
        resp.assert_text("this is a string");
    }
}
