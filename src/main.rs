use clap::Parser;
///
/// main.rs
///
///
use env_logger::{Builder, Target};
use log::error;

use rust_grpc_client::cli;

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    let args = cli::Arguments::parse();

    match &args.cmd {
        cli::SubCommand::Fleet { all: true } => {
            error!("Hello!")
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