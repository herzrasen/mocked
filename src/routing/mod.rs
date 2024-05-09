use std::sync::Arc;

use axum::Router;
use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::route::Route;

pub mod r#match;
mod matcher;
pub mod matchers;
mod method;
mod response;
pub mod route;
mod condition;
mod body;

trait Matching {
    fn matches(&self, req: &Request) -> bool;
}

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
