use std::sync::Arc;

use axum::Router;
use serde::{Deserialize, Serialize};

use crate::routing::options::Options;
use crate::routing::route::Route;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub options: Options,
    pub routes: Vec<Route>,
}

impl Config {
    pub fn router(&self) -> Router {
        let router =
            self.inherit_enable_cors()
                .routes
                .iter()
                .cloned()
                .fold(Router::new(), |acc, next| {
                    let route = Arc::new(next);
                    acc.merge(route.router())
                });
        router
    }

    fn inherit_enable_cors(&self) -> Self {
        let options = self.options.clone();
        let routes = self
            .routes
            .iter()
            .cloned()
            .map(|mut r| {
                // only update it if it is not set in it's own config
                if r.enable_cors.is_none() {
                    r.enable_cors = options.enable_cors;
                }
                r.clone()
            })
            .collect();
        Self {
            options: self.options.clone(),
            routes,
        }
    }
}

#[cfg(test)]
mod tests {
    use axum_test::http::{HeaderName, HeaderValue};
    use axum_test::TestServer;

    use crate::routing::config::Config;

    #[tokio::test]
    async fn test_create_router() {
        let config_str = r#"
                options:
                  address: localhost
                  port: 3003
                routes:
                  - path: /test
                    methods:
                      - POST
                      - PUT
                    enable_cors: true
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
