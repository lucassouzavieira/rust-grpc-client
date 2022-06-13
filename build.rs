fn main() {
    let fleet_proto_file = "./proto/fleet.proto";
    let incident_proto_file = "./proto/incident.proto";

    tonic_build::configure()
        .build_client(true)
        .compile(&[fleet_proto_file, incident_proto_file], &["."])
        .unwrap_or_else(|e| panic!("Error while compiling protos: {}", e));

    println!("cargo:rerun-if-changed={}", fleet_proto_file);
    println!("cargo:rerun-if-changed={}", incident_proto_file);
}