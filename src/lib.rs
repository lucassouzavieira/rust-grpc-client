pub mod proto;
pub mod grpc;

pub mod cli {
    use clap::{Parser, Subcommand};

    use crate::grpc;

    #[derive(Parser, Debug)]
    #[clap(author = "Lucas S. Vieira", version, about)]
    /// A gRPC CLI client
    pub struct Arguments {
        #[clap(short, long, value_parser)]
        /// gRPC server address
        pub addr: Option<String>,

        #[clap(subcommand)]
        pub cmd: SubCommand,
    }

    #[derive(Subcommand, Debug)]
    pub enum SubCommand {
        /// Handles LFB fleet info
        Fleet {
            #[clap(short, long, action)]
            /// List all vehicles available in LFB fleet
            all: bool,

            #[clap(short, long, value_parser, default_value = "")]
            /// operational status
            op_status: String,
        },

        /// Handles LFB incidents info
        Incident {
            #[clap(short, long, action)]
            /// List all incidents handled by LFB
            all: bool
        },
    }

    pub async fn list_vehicles(addr: String) {
        let results = grpc::fleet::fleet::list_vehicles(addr);
        let vx_list = results.await.into_inner();

        for vehicle in 0..vx_list.vehicles.len() {
            println!("{:?}", vx_list.vehicles[vehicle]);
        }

        println!("Total LFB fleet size: {} vehicles", vx_list.vehicles.len())
    }

    pub async fn get_vehicles_by_op_status(addr: String, status: String) {
        let results = grpc::fleet::fleet::get_vehicles_by_op_status(addr, status.clone());
        let vx_list = results.await.into_inner();

        for vehicle in 0..vx_list.vehicles.len() {
            println!("{:?}", vx_list.vehicles[vehicle]);
        }

        println!("Total LFB fleet size in {}: {} vehicles", status, vx_list.vehicles.len())
    }
}