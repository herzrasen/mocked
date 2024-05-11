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
