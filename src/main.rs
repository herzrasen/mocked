use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{middleware, Extension};
use clap::Parser;
use env_logger::Builder;
use rand::Rng;
use std::io::Write;

use crate::routing::config::Config;
use crate::routing::options::Options;

mod request;
mod routing;

#[derive(Parser, Debug)]
#[command(name = "fauxd", about = "Serve mock data")]
pub struct Cli {
    #[arg(long)]
    pub port: Option<u16>,
    #[arg(long)]
    pub host: Option<String>,
    #[arg(help = "The config file describing the routes")]
    pub config: PathBuf,
}

async fn delay_response(
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
    let mut builder = Builder::new();
    builder
        .format(|buf, rec| writeln!(buf, "{}", rec.args()))
        .filter(None, log::LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Always)
        .init();
    let args = Cli::parse();
    let config = fs::read_to_string(args.config).expect("Unable to read config");
    let config: Config = serde_yaml::from_str(&config).unwrap();

    let mut options = config.options.clone().unwrap_or_default();
    if args.host.is_some() {
        log::info!("Setting host to: {:?}", args.host);
        options.host = args.host;
    }

    if options.host.is_none() {
        eprintln!("'host' must either be set in the config or via the command line");
        std::process::exit(1);
    }

    if args.port.is_some() {
        log::info!("Setting port to: {:?}", args.port);
        options.port = args.port;
    }

    if options.port.is_none() {
        eprintln!("'port' must either be set in the config or via the command line");
        std::process::exit(1);
    }

    let router = config
        .router()
        .layer(middleware::from_fn_with_state(
            options.clone(),
            delay_response,
        ))
        .layer(Extension(options.clone()));

    match tokio::net::TcpListener::bind(format!(
        "{}:{}",
        options.clone().host.unwrap(),
        options.clone().port.unwrap()
    ))
    .await
    {
        Ok(listener) => {
            log::info!(
                "Starting server on: {}:{}",
                options.host.unwrap(),
                options.port.unwrap()
            );

            if let Err(e) = axum::serve(listener, router).await {
                log::error!("Failed to start server - {e}");
            }
        }
        Err(e) => {
            log::error!(
                "Failed to bind to: {}:{} - {e}",
                options.host.unwrap(),
                options.port.unwrap()
            );
            std::process::exit(1);
        }
    }
}
