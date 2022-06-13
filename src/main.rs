///
/// main.rs
///
use std::env;
use env_logger::{Builder, Target};

#[derive(Debug)]
struct Config {
    addr: String
}

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    let args: Vec<String> = env::args().collect();
    let config = parse_config(args);

    println!("{:?}, addr: {}", config, config.addr);
}

fn parse_config(args: Vec<String>) -> Config {
    if args.len() == 1 {
        log::error!("No server address provided!");
        return Config {addr: "No server".to_string()}
    }

    let addr = &args[1];
    Config {addr: addr.to_string()}
}
