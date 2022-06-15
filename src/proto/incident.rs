#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Incident {
    #[prost(int64, tag = "1")]
    pub number: i64,
    #[prost(string, tag = "2")]
    pub call_datetime: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub year: i32,
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub pump_count: i32,
    #[prost(int32, tag = "6")]
    pub pump_hours_total: i32,
    #[prost(int32, tag = "7")]
    pub incident_hourly_cost: i32,
    #[prost(int32, tag = "8")]
    pub incident_notional_cost: i32,
    #[prost(string, tag = "10")]
    pub animal_group: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub final_description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "12")]
    pub origin: ::core::option::Option<incident::Call>,
    #[prost(message, optional, tag = "13")]
    pub property: ::core::option::Option<incident::Property>,
    #[prost(message, optional, tag = "14")]
    pub special_service: ::core::option::Option<incident::SpecialService>,
    #[prost(message, optional, tag = "15")]
    pub ward: ::core::option::Option<incident::Ward>,
    #[prost(message, optional, tag = "16")]
    pub address: ::core::option::Option<incident::Address>,
}
/// Nested message and enum types in `Incident`.
pub mod incident {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Call {
        #[prost(string, tag = "1")]
        pub origin: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Property {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub category: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SpecialService {
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub category: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ward {
        #[prost(string, tag = "1")]
        pub code: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Address {
        #[prost(message, optional, tag = "1")]
        pub borough_info: ::core::option::Option<address::Borough>,
        #[prost(string, tag = "2")]
        pub street: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub usrn: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub postcode_district: ::prost::alloc::string::String,
        #[prost(double, tag = "5")]
        pub latitude: f64,
        #[prost(double, tag = "6")]
        pub longitude: f64,
    }
    /// Nested message and enum types in `Address`.
    pub mod address {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Borough {
            #[prost(string, tag = "1")]
            pub code: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub stn_ground_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncidentList {
    #[prost(message, repeated, tag = "1")]
    pub incidents: ::prost::alloc::vec::Vec<Incident>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIncidentsByAnimalGroupRequest {
    #[prost(string, tag = "1")]
    pub group: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod incident_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Incidents server
    #[derive(Debug, Clone)]
    pub struct IncidentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IncidentServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IncidentServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> IncidentServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            IncidentServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn list_incidents(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::IncidentList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/incident.IncidentService/ListIncidents");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_incidents_by_animal_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIncidentsByAnimalGroupRequest>,
        ) -> Result<tonic::Response<super::IncidentList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/incident.IncidentService/GetIncidentsByAnimalGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod incident_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with IncidentServiceServer.
    #[async_trait]
    pub trait IncidentService: Send + Sync + 'static {
        async fn list_incidents(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::IncidentList>, tonic::Status>;
        async fn get_incidents_by_animal_group(
            &self,
            request: tonic::Request<super::GetIncidentsByAnimalGroupRequest>,
        ) -> Result<tonic::Response<super::IncidentList>, tonic::Status>;
    }
    /// Incidents server
    #[derive(Debug)]
    pub struct IncidentServiceServer<T: IncidentService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: IncidentService> IncidentServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IncidentServiceServer<T>
    where
        T: IncidentService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/incident.IncidentService/ListIncidents" => {
                    #[allow(non_camel_case_types)]
                    struct ListIncidentsSvc<T: IncidentService>(pub Arc<T>);
                    impl<T: IncidentService> tonic::server::UnaryService<()> for ListIncidentsSvc<T> {
                        type Response = super::IncidentList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_incidents(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListIncidentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/incident.IncidentService/GetIncidentsByAnimalGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetIncidentsByAnimalGroupSvc<T: IncidentService>(pub Arc<T>);
                    impl<T: IncidentService>
                        tonic::server::UnaryService<super::GetIncidentsByAnimalGroupRequest>
                        for GetIncidentsByAnimalGroupSvc<T>
                    {
                        type Response = super::IncidentList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIncidentsByAnimalGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_incidents_by_animal_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetIncidentsByAnimalGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: IncidentService> Clone for IncidentServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: IncidentService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IncidentService> tonic::transport::NamedService for IncidentServiceServer<T> {
        const NAME: &'static str = "incident.IncidentService";
    }
}
