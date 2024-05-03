use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::rule::Rule;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rules {
    pub rules: Vec<Rule>,
}

impl Rules {
    pub fn router(&self) -> Router {
        let router = self
            .rules
            .iter().cloned()
            .fold(Router::new(), |acc, next| {
                let rule = Arc::new(next);
                println!("Merging Rule for path {}", rule.path);
                acc.merge(rule.router())
            });
        router
    }
}
