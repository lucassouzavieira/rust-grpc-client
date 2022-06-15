pub mod grpc;
pub mod proto;

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

            #[clap(short, long, value_parser)]
            /// List vehicles by operational status
            op_status: Option<String>,

            #[clap(short, long, value_parser, default_value = "0")]
            /// List vehicles by registration year
            year: i32,

            #[clap(subcommand)]
            cmd: Option<InnerCommand>,
        },

        /// Handles LFB incidents info
        Incident {
            #[clap(short, long, action)]
            /// List all incidents handled by LFB
            all: bool,

            #[clap(short, long, value_parser)]
            /// List incidents by animal group
            group: Option<String>,
        },
    }

    #[derive(Subcommand, Debug)]
    pub enum InnerCommand {
        GetStats {
            #[clap(short, long, value_parser)]
            /// Filter by vehicles maker
            maker: Option<String>,

            #[clap(short, long, value_parser)]
            /// Filter by vehicles maker
            year: Option<i32>,
        },
    }

    pub async fn list_vehicles(addr: String) {
        let results = grpc::fleet::fleet::list_vehicles(addr);
        let vx_list = results.await.into_inner();

        for vehicle in 0..vx_list.vehicles.len() {
            println!("{:#?}", vx_list.vehicles[vehicle]);
        }

        println!("Total LFB fleet size: {} vehicles", vx_list.vehicles.len())
    }

    pub async fn get_vehicles_by_op_status(addr: String, status: String) {
        let results = grpc::fleet::fleet::get_vehicles_by_op_status(addr, status.clone());
        let vx_list = results.await.into_inner();

        for vehicle in 0..vx_list.vehicles.len() {
            println!("{:#?}", vx_list.vehicles[vehicle]);
        }

        println!(
            "Total LFB fleet size in {}: {} vehicles",
            status,
            vx_list.vehicles.len()
        )
    }

    pub async fn get_vehicles_by_year(addr: String, year: i32) {
        let results = grpc::fleet::fleet::get_vehicles_by_year(addr, year);
        let vx_list = results.await.into_inner();

        for vehicle in 0..vx_list.vehicles.len() {
            println!("{:#?}", vx_list.vehicles[vehicle]);
        }

        println!(
            "Total LFB fleet size from {} year: {} vehicles",
            year,
            vx_list.vehicles.len()
        )
    }

    pub async fn get_fleet_stats(addr: String, make: Option<String>, year: Option<i32>) {
        let results = grpc::fleet::fleet::get_fleet_stats(addr, make, year);
        let stats = results.await.into_inner();

        println!("{:#?}", stats)
    }

    pub async fn list_incidents(addr: String) {
        let results = grpc::incident::incident::list_incidents(addr);
        let ix_list = results.await.into_inner();

        for ix in 0..ix_list.incidents.len() {
            println!("{:#?}", ix_list.incidents[ix]);
        }

        println!("Total: {} incidents", ix_list.incidents.len())
    }

    pub async fn get_incidents_by_animal_group(addr: String, group: String) {
        let results = grpc::incident::incident::get_incidents_by_animal_group(addr, group);
        let ix_list = results.await.into_inner();

        for ix in 0..ix_list.incidents.len() {
            println!("{:#?}", ix_list.incidents[ix]);
        }

        println!("Total: {} incidents", ix_list.incidents.len())
    }
}
