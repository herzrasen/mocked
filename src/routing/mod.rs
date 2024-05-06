use std::sync::Arc;

use axum::Router;
use serde::{Deserialize, Serialize};

use crate::request::Request;
use crate::routing::rule::Rule;

pub mod r#match;
pub mod matchers;
mod method;
pub mod rule;
mod matcher;
mod response;

trait Matching {
    fn matches(&self, req: &Request) -> bool;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Routing {
    pub rules: Vec<Rule>,
}

impl Routing {
    pub fn router(&self) -> Router {
        let router = self.rules.iter().cloned().fold(Router::new(), |acc, next| {
            let rule = Arc::new(next);
            println!("Merging Rule for path {}", rule.path);
            acc.merge(rule.router())
        });
        router
    }
}
