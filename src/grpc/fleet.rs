pub mod fleet {
    use tonic::{Request, Response};
    use tonic::transport::Channel;

    use proto::fleet::fleet_service_client::FleetServiceClient;

    use crate::proto;
    use crate::proto::fleet::VehicleList;

    pub async fn list_vehicles(server: String) -> Response<VehicleList> {
        let mut client = get_client(server).await;
        let req = Request::new({});

        match client.list_vehicles(req).await {
            Ok(response) => {
                return response;
            },
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

        return FleetServiceClient::new(channel);
    }
}
