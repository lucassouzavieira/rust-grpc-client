extern crate core;
///
/// main.rs
///
///
use clap::Parser;
use env_logger::{Builder, Target};
use rust_grpc_client::{cli, grpc};

#[tokio::main]
async fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    let args = cli::Arguments::parse();
    let server = args.addr.as_deref().unwrap();

    match &args.cmd {
        cli::SubCommand::Fleet { all: true } => {
            let results = grpc::fleet::fleet::list_vehicles(String::from(server));
            let vx_list = results.await.into_inner();

            for vehicle in 0..vx_list.vehicles.len() {
                println!("{:?}", vx_list.vehicles[vehicle]);
            }

            println!("Total LFB fleet size: {} vehicles", vx_list.vehicles.len())
        }
        cli::SubCommand::Fleet { all: false } => {
            todo!()
        }
        cli::SubCommand::Incident { all: true } => {
            todo!()
        }
        cli::SubCommand::Incident { all: false } => {
            todo!()
        }
    }
}