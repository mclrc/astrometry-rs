mod ingest_files;
mod object;

use anyhow::Result;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ingest { files: Vec<String> },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cli = Args::parse();

    match &cli.command {
        Command::Ingest { files } => ingest_files::ingest_files(files).await?,
    };

    Ok(())
}
