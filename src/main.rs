extern crate core;

use clap::Parser;
///
/// main.rs
///
///
use env_logger::{Builder, Target};
use log::error;

use rust_grpc_client::{cli, grpc};

#[tokio::main]
async fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    let args = cli::Arguments::parse();
    let mut server: String = stringify!(args.addr).parse().unwrap();

    if server.len() == 0 {
        server = "http://[::1]:9200".parse().unwrap();
    }

    match &args.cmd {
        cli::SubCommand::Fleet { all: true } => {
            let client = grpc::fleet::fleet::get_client(server.to_string());
            let client = match client.await {
                Ok(Result) => {
                    error!("Hello! Client is ok! {:?}", args)
                }
                Err(error) => {
                    panic!("Cannot connect to gRPC server: {:?}", error)
                }
            };
            println!("{:?}", args.addr);
        }
        cli::SubCommand::Fleet { all: false } => {
            error!("Hello! {:?}", args)
        }
        cli::SubCommand::Incident { all: true } => {
            todo!()
        }
        cli::SubCommand::Incident { all: false } => {
            todo!()
        }
    }
}