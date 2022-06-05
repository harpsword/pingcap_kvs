use clap::{Parser, Subcommand};
use std::env::current_dir;

#[derive(Debug, Parser)]
#[clap(name = "kvs")]
#[clap(author, version, about = "A key-value store", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Get { key: String },

    #[clap(arg_required_else_help = true)]
    Set { key: String, value: String },

    #[clap(arg_required_else_help = true, name = "rm")]
    Remove { key: String },
}

fn main() {
    let args = Cli::parse();
    // println!("command: {:#?}", &args.command);

    let path = current_dir().unwrap();
    let mut store = kvs::KvStore::open(path).unwrap();

    let result = match args.command {
        Commands::Get { key } => store.get(key).and_then(|v| {
            if v.is_none() {
                println!("Key not found");
            } else {
                println!("{}", v.unwrap());
            }
            Ok(())
        }),
        Commands::Set { key, value } => store.set(key, value),
        Commands::Remove { key } => {
            // println!("remove key {key}");
            store.remove(key)
        }
    };

    if let Err(e) = result {
        match e {
            kvs::KvsError::KeyNotFound => {
                println!("Key not found");
            }
            _ => {
                panic!("Error: {}", &e);
            }
        }
        std::process::exit(1);
    }
}
