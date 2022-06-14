pub mod fleet {
    tonic::include_proto!("fleet");
}

pub mod incident {
    tonic::include_proto!("incident");
}

pub mod cli {
    use clap::{Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[clap(author = "Lucas S. Vieira <lucassouzavieiraengcomp@gmail.com>", version, about)]
    /// A gRPC CLI client
    pub struct Arguments {
        #[clap(default_value = "localhost:9200", short, long)]
        /// gRPC server address
        addr: String,

        #[clap(subcommand)]
        cmd: SubCommand
    }

    #[derive(Subcommand, Debug)]
    pub enum SubCommand {
        /// Handles LFB fleet info
        Fleet {
            #[clap(short, long, default_value = "", )]
            all: String
        },

        /// Handles LFB incidents info
        Incident {
            #[clap(short, long, default_value = "", )]
            all: String
        },
    }
}