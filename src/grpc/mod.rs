pub mod fleet {
    use tonic::transport::Channel;
    use tonic::{Request, Response};

    use proto::fleet::fleet_service_client::FleetServiceClient;

    use crate::proto;
    use crate::proto::fleet::{
        GetFleetStatsRequest, GetFleetStatsResponse, GetVehiclesByOpStatusRequest,
        GetVehiclesByYearRequest, VehicleList,
    };

    pub async fn list_vehicles(server: String) -> Response<VehicleList> {
        let mut client = get_client(server).await;
        let req = Request::new({});

        match client.list_vehicles(req).await {
            Ok(response) => response,
            Err(e) => panic!("Something went wrong: {:?}", e),
        }
    }

    pub async fn get_vehicles_by_op_status(
        server: String,
        status: String,
    ) -> Response<VehicleList> {
        let mut client = get_client(server).await;
        let req = Request::new(GetVehiclesByOpStatusRequest { status });

        match client.get_vehicles_by_op_status(req).await {
            Ok(response) => response,
            Err(e) => panic!("Something went wrong: {:?}", e),
        }
    }

    pub async fn get_vehicles_by_year(server: String, year: i32) -> Response<VehicleList> {
        let mut client = get_client(server).await;
        let req = Request::new(GetVehiclesByYearRequest { year });

        match client.get_vehicles_by_year(req).await {
            Ok(response) => response,
            Err(e) => panic!("Something went wrong: {:?}", e),
        }
    }

    pub async fn get_fleet_stats(
        server: String,
        make: Option<String>,
        year: Option<i32>,
    ) -> Response<GetFleetStatsResponse> {
        let mut client = get_client(server).await;
        let req = Request::new(GetFleetStatsRequest { year, make });

        match client.get_fleet_stats(req).await {
            Ok(response) => response,
            Err(e) => panic!("Something went wrong: {:?}", e),
        }
    }

    async fn get_client(addr: String) -> FleetServiceClient<Channel> {
        let server = addr.clone();
        let channel = Channel::builder(server.as_str().parse().unwrap());

        let channel = match channel.connect().await {
            Ok(channel) => channel,
            Err(error) => {
                panic!("Channel creation error: {:?} {:?}", error, server)
            }
        };

        FleetServiceClient::new(channel)
    }
}

pub mod incident {
    use tonic::transport::Channel;
    use tonic::{Request, Response};

    use proto::incident::incident_service_client::IncidentServiceClient;

    use crate::proto;
    use crate::proto::incident::{GetIncidentsByAnimalGroupRequest, IncidentList};

    pub async fn list_incidents(server: String) -> Response<IncidentList> {
        let mut client = get_client(server).await;
        let req = Request::new({});

        match client.list_incidents(req).await {
            Ok(response) => {
                response
            }
            Err(e) => panic!("Something went wrong: {:?}", e),
        }
    }

    pub async fn get_incidents_by_animal_group(
        server: String,
        group: String,
    ) -> Response<IncidentList> {
        let mut client = get_client(server).await;
        let req = Request::new(GetIncidentsByAnimalGroupRequest { group });

        match client.get_incidents_by_animal_group(req).await {
            Ok(response) => {
                response
            }
            Err(e) => panic!("Something went wrong: {:?}", e),
        }
    }

    async fn get_client(addr: String) -> IncidentServiceClient<Channel> {
        let server = addr.clone();
        let channel = Channel::builder(server.as_str().parse().unwrap());

        let channel = match channel.connect().await {
            Ok(channel) => channel,
            Err(error) => {
                panic!("Channel creation error: {:?} {:?}", error, server)
            }
        };

        IncidentServiceClient::new(channel)
    }
}
