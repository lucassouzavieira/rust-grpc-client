///
/// main.rs
///
///
use env_logger::{Builder, Target};
use rust_grpc_client::cli;
use clap::Parser;

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    let args = cli::Arguments::parse();
    println!("{:?}", args)
}