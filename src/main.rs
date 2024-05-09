use std::fs;
use std::path::PathBuf;

use clap::Parser;

use crate::routing::Config;

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

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();
    let config = fs::read_to_string(args.config).expect("Unable to read config");
    let config: Config = serde_yaml::from_str(&config).unwrap();
    let config_str = serde_yaml::to_string(&config).unwrap();
    println!("{config_str}");
    let router = config.router();
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.host, args.port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
