## gRPC CLI client

A small CLI client written in Rust that queries data from [go-grpc-server](https://github.com/lucassouzavieira/go-grpc-server)

### Building
The `protos` can be obtained using a make command:  
`make proto-get`

After getting the proto files, you can use `cargo build` to build the project.

### Usage
```
rust-grpc-client 0.1.0
A gRPC CLI client

USAGE:
    rust-grpc-client [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a, --addr <ADDR>    gRPC server address
    -h, --help           Print help information
    -V, --version        Print version information

SUBCOMMANDS:
    fleet       Handles LFB fleet info
    help        Print this message or the help of the given subcommand(s)
    incident    Handles LFB incidents info
```