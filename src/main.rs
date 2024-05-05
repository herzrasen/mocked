use std::fs;

use clap::Parser;

use crate::cli::Cli;
use crate::rules::Rules;

mod cli;
mod matcher;
mod method;
mod response;
mod rule;
mod rules;
mod matchers;
mod r#match;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();
    let config = fs::read_to_string(args.config).expect("Unable to read config");
    let rules: Rules = serde_yaml::from_str(&config).unwrap();
    let router = rules.router();
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
