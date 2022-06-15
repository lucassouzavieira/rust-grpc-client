///
/// main.rs
///
///
extern crate core;

use clap::Parser;
use env_logger::{Builder, Target};

use rust_grpc_client::cli;

#[tokio::main]
async fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    let args = cli::Arguments::parse();
    let server = args.addr.as_deref().unwrap();

    match &args.cmd {
        cli::SubCommand::Fleet { all: true, ..} => {
            cli::list_vehicles(String::from(server)).await;
        }
        cli::SubCommand::Fleet { op_status, .. } => {
            cli::get_vehicles_by_op_status(String::from(server), String::from(op_status)).await;
        }
        cli::SubCommand::Incident { all: true } => {
            todo!()
        }
        cli::SubCommand::Incident { all: false } => {
            todo!()
        }
    }
}