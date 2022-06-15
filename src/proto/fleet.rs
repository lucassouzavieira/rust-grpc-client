#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vehicle {
    #[prost(string, tag = "1")]
    pub fleet_number: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub operational_status: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub lfb: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub make: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub model: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub category: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub registration_year: i32,
    #[prost(int32, tag = "9")]
    pub life: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleRequest {
    #[prost(string, tag = "2")]
    pub operational_status: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub lfb: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub make: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub model: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub category: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub registration_year: i32,
    #[prost(int32, tag = "9")]
    pub life: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVehiclesByOpStatusRequest {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVehiclesByYearRequest {
    #[prost(int32, tag = "1")]
    pub year: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleResponse {
    #[prost(message, optional, tag = "1")]
    pub vehicle: ::core::option::Option<Vehicle>,
    #[prost(bool, tag = "2")]
    pub created: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleList {
    #[prost(message, repeated, tag = "1")]
    pub vehicles: ::prost::alloc::vec::Vec<Vehicle>,
}
/// Generated client implementations.
pub mod fleet_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Fleet server
    #[derive(Debug, Clone)]
    pub struct FleetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FleetServiceClient<tonic::transport::Channel> {
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
    impl<T> FleetServiceClient<T>
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
        ) -> FleetServiceClient<InterceptedService<T, F>>
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
            FleetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::VehicleList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fleet.FleetService/ListVehicles");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::VehicleRequest>,
        ) -> Result<tonic::Response<super::VehicleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/fleet.FleetService/AddVehicle");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_vehicles_by_op_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVehiclesByOpStatusRequest>,
        ) -> Result<tonic::Response<super::VehicleList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fleet.FleetService/GetVehiclesByOpStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_vehicles_by_year(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVehiclesByYearRequest>,
        ) -> Result<tonic::Response<super::VehicleList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/fleet.FleetService/GetVehiclesByYear");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod fleet_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with FleetServiceServer.
    #[async_trait]
    pub trait FleetService: Send + Sync + 'static {
        async fn list_vehicles(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::VehicleList>, tonic::Status>;
        async fn add_vehicle(
            &self,
            request: tonic::Request<super::VehicleRequest>,
        ) -> Result<tonic::Response<super::VehicleResponse>, tonic::Status>;
        async fn get_vehicles_by_op_status(
            &self,
            request: tonic::Request<super::GetVehiclesByOpStatusRequest>,
        ) -> Result<tonic::Response<super::VehicleList>, tonic::Status>;
        async fn get_vehicles_by_year(
            &self,
            request: tonic::Request<super::GetVehiclesByYearRequest>,
        ) -> Result<tonic::Response<super::VehicleList>, tonic::Status>;
    }
    /// Fleet server
    #[derive(Debug)]
    pub struct FleetServiceServer<T: FleetService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: FleetService> FleetServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FleetServiceServer<T>
    where
        T: FleetService,
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
                "/fleet.FleetService/ListVehicles" => {
                    #[allow(non_camel_case_types)]
                    struct ListVehiclesSvc<T: FleetService>(pub Arc<T>);
                    impl<T: FleetService> tonic::server::UnaryService<()> for ListVehiclesSvc<T> {
                        type Response = super::VehicleList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_vehicles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListVehiclesSvc(inner);
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
                "/fleet.FleetService/AddVehicle" => {
                    #[allow(non_camel_case_types)]
                    struct AddVehicleSvc<T: FleetService>(pub Arc<T>);
                    impl<T: FleetService> tonic::server::UnaryService<super::VehicleRequest> for AddVehicleSvc<T> {
                        type Response = super::VehicleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VehicleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_vehicle(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddVehicleSvc(inner);
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
                "/fleet.FleetService/GetVehiclesByOpStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetVehiclesByOpStatusSvc<T: FleetService>(pub Arc<T>);
                    impl<T: FleetService>
                        tonic::server::UnaryService<super::GetVehiclesByOpStatusRequest>
                        for GetVehiclesByOpStatusSvc<T>
                    {
                        type Response = super::VehicleList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVehiclesByOpStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_vehicles_by_op_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVehiclesByOpStatusSvc(inner);
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
                "/fleet.FleetService/GetVehiclesByYear" => {
                    #[allow(non_camel_case_types)]
                    struct GetVehiclesByYearSvc<T: FleetService>(pub Arc<T>);
                    impl<T: FleetService>
                        tonic::server::UnaryService<super::GetVehiclesByYearRequest>
                        for GetVehiclesByYearSvc<T>
                    {
                        type Response = super::VehicleList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVehiclesByYearRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_vehicles_by_year(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVehiclesByYearSvc(inner);
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
    impl<T: FleetService> Clone for FleetServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: FleetService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: FleetService> tonic::transport::NamedService for FleetServiceServer<T> {
        const NAME: &'static str = "fleet.FleetService";
    }
}
