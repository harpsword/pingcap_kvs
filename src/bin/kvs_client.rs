#![feature(async_closure)]

use std::env::current_dir;
use std::net::SocketAddr;
use std::process::exit;

use clap::{Parser, Subcommand};
use kvs::KvsEngine;
use kvs::Result;
use pilota::lazy_static::lazy_static;
use tracing::debug;
use tracing::{event, info, Level};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, default_value = "127.0.0.1:8080")]
    addr: String,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
}

#[volo::main]
async fn main() {
    init_tracing();

    let cli = Cli::parse();
    info!("{}", cli.addr);

    let addr: SocketAddr = cli.addr.parse().unwrap();
    let client = volo_gen::kvs::KvsServiceClientBuilder::new("kvs")
        .address(addr)
        .build();


    let result = async || -> Result<()> {
        match &cli.command {
            Commands::Set { key, value } => {
                debug!("set key: {}, value: {}", key, value);
                let req = volo_gen::kvs::SetRequest{key: key.to_owned().into(), value: value.to_owned().into()};
                let resp = client.set(req).await?;
                debug!("set response: {:?}", &resp);
                Ok(())
            }
            Commands::Get { key } => {
                debug!("key: {}", key);
                let req = volo_gen::kvs::GetRequest{key: key.to_owned().into()};
                let resp = client.get(req).await?;
                debug!("get response: {:?}", &resp);
                Ok(())
            }
            Commands::Rm { key } => {
                debug!("rm key: {}", key);
                let req = volo_gen::kvs::RemoveRequest{key: key.to_owned().into()};
                let resp = client.remove(req).await?;
                debug!("remove response: {:?}", &resp);
                Ok(())
            }
        }
    }().await;

    if let Err(e) = result {
        println!("{}", e);
        exit(1);
    }
}
