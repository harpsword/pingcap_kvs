use std::env::current_dir;

use clap::{Parser, Subcommand};
use kvs::KvStorage;
use kvs::Result;
use serde::de::value;
use tracing::{event, info, Level};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

fn main() {
    init_tracing();

    let cli = Cli::parse();

    let result = || -> Result<()> {
        let mut kv_store = kvs::KvStore::open(current_dir()?)?;
        match &cli.command {
            Commands::Set { key, value } => {
                info!("set key: {}, value: {}", key, value);
                kv_store.set(key.to_owned(), value.to_owned())
            }
            Commands::Get { key } => {
                event!(Level::INFO, "key: {}", key);
                let value = kv_store.get(key.to_string())?;
                match value {
                    Some(value) => {
                        println!("{}", value);
                        Ok(())
                    }
                    None => {
                        println!("Key not found");
                        Ok(())
                    }
                }
            }
            Commands::Rm { key } => {
                event!(Level::INFO, "rm key: {}", key);
                kv_store.remove(key.to_string())
            }
        }
    }();
}
