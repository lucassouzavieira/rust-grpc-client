mod proto;

pub mod grpc {
    use crate::proto;
    use proto::fleet::fleet_service_client::FleetServiceClient;

    pub fn get_client(){
        let mut client = FleetServiceClient::connect("localhost:9200").await?;
        println!("{:?}", client)
    }
}

pub mod cli {
    use clap::{Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[clap(author = "Lucas S. Vieira <lucassouzavieiraengcomp@gmail.com>", version, about)]
    /// A gRPC CLI client
    pub struct Arguments {
        #[clap(short, long, value_parser)]
        /// gRPC server address
        pub addr: Option<String>,

        #[clap(subcommand)]
        pub cmd: SubCommand
    }

    #[derive(Subcommand, Debug)]
    pub enum SubCommand {
        /// Handles LFB fleet info
        Fleet {
            #[clap(short, long, action)]
            /// List all vehicles available in LFB fleet
            all: bool
        },

        /// Handles LFB incidents info
        Incident {
            #[clap(short, long, action)]
            /// List all incidents handled by LFB
            all: bool
        },
    }
}