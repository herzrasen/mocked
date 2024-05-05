use std::path::PathBuf;

use clap::Parser;

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
