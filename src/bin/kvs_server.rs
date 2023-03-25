use clap::Parser;



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value="127.0.0.1")]
    addr: String,

    #[arg(long, default_value="kvs_store")]
    engine: String,
}


fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

fn main() {
    init_tracing();

    let args = Args::parse();
}