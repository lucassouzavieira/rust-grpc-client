pub mod fleet {
    use tonic::transport::{Channel, Error, Uri};

    use proto::fleet::fleet_service_client::FleetServiceClient;

    use crate::proto;

    pub async fn get_client(addr: String) -> FleetServiceClient<Channel> {
        let server = addr.clone();
        let channel = Channel::builder(server.as_str().parse().unwrap());

        let channel = match channel.connect().await {
            Ok(channel) => channel,
            Err(error) => {
                panic!("Channel creation error: {:?}", error)
            }
        };

        return FleetServiceClient::new(channel);
    }
}
