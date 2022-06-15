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
                return response;
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
                return response;
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

        return IncidentServiceClient::new(channel);
    }
}
