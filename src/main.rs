use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::process::Output;
use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{middleware, Extension};
use clap::Parser;
use rand::Rng;

use crate::routing::config::Config;
use crate::routing::options::Options;

mod request;
mod routing;

#[derive(Parser, Debug)]
#[command(name = "mockd", about = "Serve mock data")]
pub struct Cli {
    #[arg(long)]
    pub port: u16,
    #[arg(long)]
    pub host: String,
    #[arg(help = "The config file describing the routes")]
    pub config: PathBuf,
}

async fn sleep_before_response(
    Extension(options): Extension<Options>,
    req: Request,
    next: Next,
) -> Response {
    let min = options.min_response_delay_ms.unwrap_or(0);
    let max = options.max_response_delay_ms.unwrap_or(min);
    let delay = rand::thread_rng().gen_range(min..=max);
    log::info!("Sleeping for: {delay}ms");

    tokio::time::sleep(Duration::from_millis(delay)).await;

    let resp = next.run(req).await;
    resp
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();
    let config = fs::read_to_string(args.config).expect("Unable to read config");
    let config: Config = serde_yaml::from_str(&config).unwrap();

    let options = config.options.clone().unwrap_or(Options {
        min_response_delay_ms: None,
        max_response_delay_ms: None,
    });

    let router = config
        .router()
        .layer(middleware::from_fn_with_state(
            options.clone(),
            sleep_before_response,
        ))
        .layer(Extension(options));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
