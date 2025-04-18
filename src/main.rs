use std::path::PathBuf;
use std::time::Duration;

use axum::extract::Request;
use axum::http::Method;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use clap::{ArgAction, Parser, Subcommand};
use env_logger::Builder;
use rand::Rng;
use std::io::Write;

use crate::routing::options::Options;

mod init;
mod request;
mod routing;
mod start;

#[derive(Parser, Debug)]
#[command(name = "mocked", about = "Serve mock data")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        #[arg(long, default_value_t = 3003)]
        port: u16,
        #[arg(long, default_value_t=String::from("localhost"))]
        address: String,
        #[arg(long, default_value_t = true, action=ArgAction::Set)]
        enable_cors: bool,
        #[arg(long, default_value_t = 100)]
        min_response_delay_ms: u64,
        #[arg(long, default_value_t = 500)]
        max_response_delay_ms: u64,
        #[arg(long, default_value_t=false, action=ArgAction::Set)]
        without_example_routes: bool,
        #[arg(long, default_value_t=String::from("mocked.yml"))]
        path: String,
    },
    Start {
        #[arg(help = "The config file describing the routes")]
        config: PathBuf,
    },
}

async fn delay_response(
    Extension(options): Extension<Options>,
    req: Request,
    next: Next,
) -> Response {
    if req.method() != Method::OPTIONS {
        let min = options.min_response_delay_ms.unwrap_or(0);
        let max = options.max_response_delay_ms.unwrap_or(min);
        let delay = rand::rng().random_range(min..=max);
        log::info!("Delaying response for: {delay}ms");
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }
    next.run(req).await
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

    match args.command {
        Commands::Start { config } => start::start(config).await,
        Commands::Init {
            port,
            address,
            enable_cors,
            min_response_delay_ms,
            max_response_delay_ms,
            without_example_routes,
            path,
        } => {
            init::init(
                port,
                address,
                enable_cors,
                min_response_delay_ms,
                max_response_delay_ms,
                without_example_routes,
                path,
            )
            .await
        }
    }
}
