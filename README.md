## gRPC CLI client

A small CLI client written in Rust that queries data from [go-grpc-server](https://github.com/lucassouzavieira/go-grpc-server)

### Building
The `protos` can be obtained using a make command:  

`make proto-get`

After getting the proto files, you can use `cargo build` to build the project.

### Usage

In order to be able to query info from [server](https://github.com/lucassouzavieira/go-grpc-server), you must have it up and running.
Follow instructions on [go-grpc-server](https://github.com/lucassouzavieira/go-grpc-server) repository in order to have it running. 
After that, you use `--addr` option to tell CLI the server address to make perform requests.

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

#### Examples

Get all animal incidents caused by snakes

```bash
./rust-grpc-client --addr http://127.0.0.1:9200 incident -g Snake

.
.
.

Incident { number: 5705323042022, call_datetime: "23/04/2022 08:29", year: 2022, r#type: "Special Service", pump_count: 1, pump_hours_total: 1, incident_hourly_cost: 0, incident_notional_cost: 0, animal_group: "Snake", 
  final_description: "SNAKE TRAPPED IN DOOR", origin: Some(Call { origin: "Person (mobile)" }), property: Some(Property { r#type: "Purpose Built Flats/Maisonettes - 4 to 9 storeys ", category: "Dwelling" }), 
  special_service: Some(SpecialService { r#type: "Animal assistance involving wild animal - Other action", category: "Other animal assistance" }), ward: Some(Ward { code: "E05000225", name: "PENINSULA" }), 
  address: Some(Address { borough_info: Some(Borough { code: "E09000011", name: "GREENWICH", stn_ground_name: "East Greenwich" }), street: "NULL", usrn: "20802888", postcode_district: "SE7", latitude: 0.0, longitude: 0.0 }) }

Total: 31 incidents
```