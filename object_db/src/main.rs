mod ingest_files;
mod object;
mod schema;

use anyhow::Result;

use clap::{Parser, Subcommand};
use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use std::env;

fn establish_connection() -> Result<PgConnection> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Ok(PgConnection::establish(&database_url)?)
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ingest { files: Vec<String> },
}

fn main() -> Result<()> {
    dotenv().ok();

    let cli = Args::parse();
    let mut db = establish_connection()?;

    match &cli.command {
        Command::Ingest { files } => ingest_files::ingest_files(&mut db, files)?,
    };

    Ok(())
}
