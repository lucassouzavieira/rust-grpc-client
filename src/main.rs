///
/// main.rs
///
///
use env_logger::{Builder, Target};
use clap::{Arg, Command};
use log::{error};

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    let cmd = Command::new("fleet")
        .about("Returns info about LFB vehicles")
        .subcommand_required(true)
        .author("Lucas S. Vieira <lucassouzavieiraengcomp@gmail.com>")
        .subcommand(
            Command::new("list")
                .short_flag('L')
                .long_flag("list")
                .about("List vehicles")
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("List all vehicles")
                        .takes_value(false)
                )
        ).arg(
            Arg::new("server").short('s')
                .long("server")
                .help("gRPC server address")
                .takes_value(true)
                .default_value("localhost:9200")
        ).arg_required_else_help(true);

    let matches = cmd.get_matches();
    // let args = cmd.get_arguments();

    match matches.subcommand() {
        Some(("list", list_matches)) => {
            if list_matches.contains_id("all") {
                // TODO implement
                error!("List all vehicles action");
                // for arg in args {
                //     println!("{}", arg)
                // }
            }
        }
        _ => unreachable!(), // all subcommands failed
    }
}