///
/// main.rs
///
extern crate core;

use std::string::String;

use clap::Parser;
use env_logger::{Builder, Target};
use log::info;

use rust_grpc_client::cli;

#[tokio::main]
async fn main() {
    Builder::new().target(Target::Stdout).init();

    let args = cli::Arguments::parse();
    let server = args.addr.as_deref().unwrap();

    match &args.cmd {
        cli::SubCommand::Fleet {
            all,
            op_status,
            year,
            cmd,
        } => {
            if *all {
                cli::list_vehicles(String::from(server)).await;
                return;
            }

            if *op_status != None {
                let status = op_status.as_deref().unwrap();
                cli::get_vehicles_by_op_status(String::from(server), String::from(status)).await;
                return;
            }

            if *year != 0 {
                cli::get_vehicles_by_year(String::from(server), *year).await;
            }

            match &*cmd {
                None => {}
                Some(v) => match v {
                    cli::InnerCommand::GetStats { maker, year } => {
                        let mut maker_str: &String = &"".to_string();
                        let mut reg_year: &i32 = &0;

                        match maker {
                            None => info!("Maker not set..."),
                            Some(v) => maker_str = v,
                        }

                        match year {
                            None => info!("Year not set..."),
                            Some(y) => reg_year = y,
                        }

                        cli::get_fleet_stats(
                            String::from(server),
                            Some(maker_str.to_string()),
                            Some(reg_year.clone()),
                        )
                        .await
                    }
                },
            }
        }
        cli::SubCommand::Incident { all, group } => {
            if *all {
                cli::list_incidents(String::from(server)).await;
                return;
            }

            if *group != None {
                let group = group.as_deref().unwrap();
                cli::get_incidents_by_animal_group(String::from(server), String::from(group)).await;
            }
        }
    }
}
