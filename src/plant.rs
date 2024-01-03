#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plant {
    #[prost(message, optional, tag = "1")]
    pub identifier: ::core::option::Option<PlantIdentifier>,
    #[prost(message, optional, tag = "2")]
    pub information: ::core::option::Option<PlantInformation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlantIdentifier {
    #[prost(string, tag = "1")]
    pub sku: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub device_identifier: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlantInformation {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub last_watered: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub last_health_check: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "4")]
    pub last_identification: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub identified_species_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlantUpdateRequest {
    #[prost(message, optional, tag = "1")]
    pub identifier: ::core::option::Option<PlantIdentifier>,
    #[prost(message, optional, tag = "2")]
    pub information: ::core::option::Option<PlantInformation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlantResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlantUpdateResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfPlants {
    #[prost(message, repeated, tag = "1")]
    pub plants: ::prost::alloc::vec::Vec<Plant>,
}
/// Health check info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckDataRequest {
    #[prost(message, optional, tag = "1")]
    pub identifier: ::core::option::Option<PlantIdentifier>,
    #[prost(string, tag = "2")]
    pub health_check_information: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckDataResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckInformation {
    #[prost(double, tag = "1")]
    pub probability: f64,
    #[prost(message, optional, tag = "2")]
    pub historical_probabilities: ::core::option::Option<HistoricalProbabilities>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalProbabilities {
    #[prost(message, repeated, tag = "1")]
    pub probabilities: ::prost::alloc::vec::Vec<Probabilities>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Probabilities {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(double, tag = "3")]
    pub probability: f64,
    #[prost(int64, tag = "4")]
    pub date: i64,
}
/// Generated client implementations.
pub mod plant_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PlantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlantServiceClient<tonic::transport::Channel> {
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
    impl<T> PlantServiceClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PlantServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlantServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Create plant
        pub async fn add(
            &mut self,
            request: impl tonic::IntoRequest<super::Plant>,
        ) -> Result<tonic::Response<super::PlantResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/plant.PlantService/Add");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Remove plant
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::PlantResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plant.PlantService/Remove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get plant
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::Plant>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/plant.PlantService/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a list of plants that need to be watered (for APNs microservice)
        pub async fn get_watered(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::ListOfPlants>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plant.PlantService/GetWatered",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update plant schedule/health check/id time
        pub async fn update_plant(
            &mut self,
            request: impl tonic::IntoRequest<super::PlantUpdateRequest>,
        ) -> Result<tonic::Response<super::PlantUpdateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plant.PlantService/UpdatePlant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Save JSON health check
        pub async fn save_health_check_data(
            &mut self,
            request: impl tonic::IntoRequest<super::HealthCheckDataRequest>,
        ) -> Result<tonic::Response<super::HealthCheckDataResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plant.PlantService/SaveHealthCheckData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Caching
        pub async fn identification_request(
            &mut self,
            request: impl tonic::IntoRequest<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::PlantInformation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plant.PlantService/IdentificationRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn health_check_request(
            &mut self,
            request: impl tonic::IntoRequest<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::HealthCheckInformation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/plant.PlantService/HealthCheckRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod plant_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlantServiceServer.
    #[async_trait]
    pub trait PlantService: Send + Sync + 'static {
        /// Create plant
        async fn add(
            &self,
            request: tonic::Request<super::Plant>,
        ) -> Result<tonic::Response<super::PlantResponse>, tonic::Status>;
        /// Remove plant
        async fn remove(
            &self,
            request: tonic::Request<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::PlantResponse>, tonic::Status>;
        /// Get plant
        async fn get(
            &self,
            request: tonic::Request<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::Plant>, tonic::Status>;
        /// Get a list of plants that need to be watered (for APNs microservice)
        async fn get_watered(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::ListOfPlants>, tonic::Status>;
        /// Update plant schedule/health check/id time
        async fn update_plant(
            &self,
            request: tonic::Request<super::PlantUpdateRequest>,
        ) -> Result<tonic::Response<super::PlantUpdateResponse>, tonic::Status>;
        /// Save JSON health check
        async fn save_health_check_data(
            &self,
            request: tonic::Request<super::HealthCheckDataRequest>,
        ) -> Result<tonic::Response<super::HealthCheckDataResponse>, tonic::Status>;
        /// Caching
        async fn identification_request(
            &self,
            request: tonic::Request<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::PlantInformation>, tonic::Status>;
        async fn health_check_request(
            &self,
            request: tonic::Request<super::PlantIdentifier>,
        ) -> Result<tonic::Response<super::HealthCheckInformation>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PlantServiceServer<T: PlantService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlantService> PlantServiceServer<T> {
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlantServiceServer<T>
    where
        T: PlantService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/plant.PlantService/Add" => {
                    #[allow(non_camel_case_types)]
                    struct AddSvc<T: PlantService>(pub Arc<T>);
                    impl<T: PlantService> tonic::server::UnaryService<super::Plant>
                    for AddSvc<T> {
                        type Response = super::PlantResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Plant>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plant.PlantService/Remove" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveSvc<T: PlantService>(pub Arc<T>);
                    impl<
                        T: PlantService,
                    > tonic::server::UnaryService<super::PlantIdentifier>
                    for RemoveSvc<T> {
                        type Response = super::PlantResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlantIdentifier>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plant.PlantService/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: PlantService>(pub Arc<T>);
                    impl<
                        T: PlantService,
                    > tonic::server::UnaryService<super::PlantIdentifier> for GetSvc<T> {
                        type Response = super::Plant;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlantIdentifier>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plant.PlantService/GetWatered" => {
                    #[allow(non_camel_case_types)]
                    struct GetWateredSvc<T: PlantService>(pub Arc<T>);
                    impl<T: PlantService> tonic::server::UnaryService<()>
                    for GetWateredSvc<T> {
                        type Response = super::ListOfPlants;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_watered(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWateredSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plant.PlantService/UpdatePlant" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePlantSvc<T: PlantService>(pub Arc<T>);
                    impl<
                        T: PlantService,
                    > tonic::server::UnaryService<super::PlantUpdateRequest>
                    for UpdatePlantSvc<T> {
                        type Response = super::PlantUpdateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlantUpdateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_plant(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdatePlantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plant.PlantService/SaveHealthCheckData" => {
                    #[allow(non_camel_case_types)]
                    struct SaveHealthCheckDataSvc<T: PlantService>(pub Arc<T>);
                    impl<
                        T: PlantService,
                    > tonic::server::UnaryService<super::HealthCheckDataRequest>
                    for SaveHealthCheckDataSvc<T> {
                        type Response = super::HealthCheckDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HealthCheckDataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).save_health_check_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SaveHealthCheckDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plant.PlantService/IdentificationRequest" => {
                    #[allow(non_camel_case_types)]
                    struct IdentificationRequestSvc<T: PlantService>(pub Arc<T>);
                    impl<
                        T: PlantService,
                    > tonic::server::UnaryService<super::PlantIdentifier>
                    for IdentificationRequestSvc<T> {
                        type Response = super::PlantInformation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlantIdentifier>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).identification_request(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IdentificationRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/plant.PlantService/HealthCheckRequest" => {
                    #[allow(non_camel_case_types)]
                    struct HealthCheckRequestSvc<T: PlantService>(pub Arc<T>);
                    impl<
                        T: PlantService,
                    > tonic::server::UnaryService<super::PlantIdentifier>
                    for HealthCheckRequestSvc<T> {
                        type Response = super::HealthCheckInformation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlantIdentifier>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).health_check_request(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HealthCheckRequestSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlantService> Clone for PlantServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PlantService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlantService> tonic::server::NamedService for PlantServiceServer<T> {
        const NAME: &'static str = "plant.PlantService";
    }
}
