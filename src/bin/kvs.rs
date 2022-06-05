use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "kvs")]
#[clap(author, version, about = "A key-value store", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    comamnd: Commands,
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

    let store = kvs::KvStore::new();

    match args.comamnd {
        Commands::Get { key } => {
            println!("get key: {key}");
            eprint!("unimplemented");
            std::process::exit(1);
        }
        Commands::Set { key, value } => {
            println!("set key {key}, value {value}");
            eprint!("unimplemented");
            std::process::exit(1);
        }
        Commands::Remove { key } => {
            println!("remove key {key}");
            eprint!("unimplemented");
            std::process::exit(1);
        }
    }

    println!("Hello, World!");
}
