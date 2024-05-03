use std::fmt::Display;
use std::fs;
use std::future::Future;

use axum::extract::Request;
use clap::Parser;
use serde::Serialize;

use crate::cli::Cli;
use crate::rules::Rules;

mod cli;
mod rule;
mod rules;

async fn respond(request: Request) -> String {
    println!("{:?}", request);
    format!("{:?}", request).to_string()
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let config = fs::read_to_string(args.config).expect("Unable to read config");
    let rules: Rules = serde_yaml::from_str(&config).unwrap();
    let router = rules.router();
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
