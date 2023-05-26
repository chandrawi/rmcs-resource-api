#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSchema {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[prost(uint32, tag = "2")]
    pub model_id: u32,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(uint32, tag = "4")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "5")]
    pub data_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "super::common::DataType", repeated, tag = "6")]
    pub data_type: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataId {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[prost(uint32, tag = "2")]
    pub model_id: u32,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(uint32, tag = "4")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataTime {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[prost(uint32, tag = "2")]
    pub model_id: u32,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRange {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[prost(uint32, tag = "2")]
    pub model_id: u32,
    #[prost(int64, tag = "3")]
    pub begin: i64,
    #[prost(int64, tag = "4")]
    pub end: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataNumber {
    #[prost(uint64, tag = "1")]
    pub device_id: u64,
    #[prost(uint32, tag = "2")]
    pub model_id: u32,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(uint32, tag = "4")]
    pub number: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelId {
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataModel {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(enumeration = "super::common::DataIndexing", tag = "2")]
    pub indexing: i32,
    #[prost(enumeration = "super::common::DataType", repeated, tag = "3")]
    pub types: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSchemaModel {
    #[prost(message, optional, tag = "1")]
    pub model: ::core::option::Option<DataModel>,
    #[prost(uint64, tag = "2")]
    pub device_id: u64,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(uint32, tag = "4")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "5")]
    pub data_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "super::common::DataType", repeated, tag = "6")]
    pub data_type: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataIdModel {
    #[prost(message, optional, tag = "1")]
    pub model: ::core::option::Option<DataModel>,
    #[prost(uint64, tag = "2")]
    pub device_id: u64,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(uint32, tag = "4")]
    pub index: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataTimeModel {
    #[prost(message, optional, tag = "1")]
    pub model: ::core::option::Option<DataModel>,
    #[prost(uint64, tag = "2")]
    pub device_id: u64,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRangeModel {
    #[prost(message, optional, tag = "1")]
    pub model: ::core::option::Option<DataModel>,
    #[prost(uint64, tag = "2")]
    pub device_id: u64,
    #[prost(int64, tag = "3")]
    pub begin: i64,
    #[prost(int64, tag = "4")]
    pub end: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataNumberModel {
    #[prost(message, optional, tag = "1")]
    pub model: ::core::option::Option<DataModel>,
    #[prost(uint64, tag = "2")]
    pub device_id: u64,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(uint32, tag = "4")]
    pub number: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<DataSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<DataSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataModelResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<DataModel>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataChangeResponse {
    #[prost(enumeration = "super::common::ResponseStatus", tag = "1")]
    pub status: i32,
}
/// Generated client implementations.
pub mod data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DataServiceClient<T>
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
        ) -> DataServiceClient<InterceptedService<T, F>>
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
            DataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn read_data(
            &mut self,
            request: impl tonic::IntoRequest<super::DataId>,
        ) -> std::result::Result<
            tonic::Response<super::DataReadResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ReadData",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("data.DataService", "ReadData"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_by_time(
            &mut self,
            request: impl tonic::IntoRequest<super::DataTime>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataByTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "ListDataByTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_by_last_time(
            &mut self,
            request: impl tonic::IntoRequest<super::DataTime>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataByLastTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "ListDataByLastTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_by_range_time(
            &mut self,
            request: impl tonic::IntoRequest<super::DataRange>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataByRangeTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "ListDataByRangeTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_by_number_before(
            &mut self,
            request: impl tonic::IntoRequest<super::DataNumber>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataByNumberBefore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "ListDataByNumberBefore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_by_number_after(
            &mut self,
            request: impl tonic::IntoRequest<super::DataNumber>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataByNumberAfter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "ListDataByNumberAfter"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_data_model(
            &mut self,
            request: impl tonic::IntoRequest<super::ModelId>,
        ) -> std::result::Result<
            tonic::Response<super::DataModelResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/GetDataModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "GetDataModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_data_with_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DataIdModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataReadResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ReadDataWithModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "ReadDataWithModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_with_model_by_time(
            &mut self,
            request: impl tonic::IntoRequest<super::DataTimeModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataWithModelByTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "ListDataWithModelByTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_with_model_by_last_time(
            &mut self,
            request: impl tonic::IntoRequest<super::DataTimeModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataWithModelByLastTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("data.DataService", "ListDataWithModelByLastTime"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_with_model_by_range_time(
            &mut self,
            request: impl tonic::IntoRequest<super::DataRangeModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataWithModelByRangeTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("data.DataService", "ListDataWithModelByRangeTime"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_with_model_by_number_before(
            &mut self,
            request: impl tonic::IntoRequest<super::DataNumberModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataWithModelByNumberBefore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "data.DataService",
                        "ListDataWithModelByNumberBefore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_data_with_model_by_number_after(
            &mut self,
            request: impl tonic::IntoRequest<super::DataNumberModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/ListDataWithModelByNumberAfter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("data.DataService", "ListDataWithModelByNumberAfter"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_data(
            &mut self,
            request: impl tonic::IntoRequest<super::DataSchema>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/CreateData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "CreateData"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_data_with_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DataSchemaModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/CreateDataWithModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "CreateDataWithModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_data(
            &mut self,
            request: impl tonic::IntoRequest<super::DataId>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/DeleteData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "DeleteData"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_data_with_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DataIdModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        > {
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
                "/data.DataService/DeleteDataWithModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("data.DataService", "DeleteDataWithModel"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod data_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataServiceServer.
    #[async_trait]
    pub trait DataService: Send + Sync + 'static {
        async fn read_data(
            &self,
            request: tonic::Request<super::DataId>,
        ) -> std::result::Result<
            tonic::Response<super::DataReadResponse>,
            tonic::Status,
        >;
        async fn list_data_by_time(
            &self,
            request: tonic::Request<super::DataTime>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_by_last_time(
            &self,
            request: tonic::Request<super::DataTime>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_by_range_time(
            &self,
            request: tonic::Request<super::DataRange>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_by_number_before(
            &self,
            request: tonic::Request<super::DataNumber>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_by_number_after(
            &self,
            request: tonic::Request<super::DataNumber>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn get_data_model(
            &self,
            request: tonic::Request<super::ModelId>,
        ) -> std::result::Result<
            tonic::Response<super::DataModelResponse>,
            tonic::Status,
        >;
        async fn read_data_with_model(
            &self,
            request: tonic::Request<super::DataIdModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataReadResponse>,
            tonic::Status,
        >;
        async fn list_data_with_model_by_time(
            &self,
            request: tonic::Request<super::DataTimeModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_with_model_by_last_time(
            &self,
            request: tonic::Request<super::DataTimeModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_with_model_by_range_time(
            &self,
            request: tonic::Request<super::DataRangeModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_with_model_by_number_before(
            &self,
            request: tonic::Request<super::DataNumberModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn list_data_with_model_by_number_after(
            &self,
            request: tonic::Request<super::DataNumberModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataListResponse>,
            tonic::Status,
        >;
        async fn create_data(
            &self,
            request: tonic::Request<super::DataSchema>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        >;
        async fn create_data_with_model(
            &self,
            request: tonic::Request<super::DataSchemaModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        >;
        async fn delete_data(
            &self,
            request: tonic::Request<super::DataId>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        >;
        async fn delete_data_with_model(
            &self,
            request: tonic::Request<super::DataIdModel>,
        ) -> std::result::Result<
            tonic::Response<super::DataChangeResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DataServiceServer<T: DataService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataService> DataServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DataServiceServer<T>
    where
        T: DataService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/data.DataService/ReadData" => {
                    #[allow(non_camel_case_types)]
                    struct ReadDataSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataId>
                    for ReadDataSvc<T> {
                        type Response = super::DataReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).read_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataByTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataByTimeSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataTime>
                    for ListDataByTimeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataTime>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_by_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataByTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataByLastTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataByLastTimeSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataTime>
                    for ListDataByLastTimeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataTime>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_by_last_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataByLastTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataByRangeTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataByRangeTimeSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataRange>
                    for ListDataByRangeTimeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataRange>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_by_range_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataByRangeTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataByNumberBefore" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataByNumberBeforeSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataNumber>
                    for ListDataByNumberBeforeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataNumber>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_by_number_before(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataByNumberBeforeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataByNumberAfter" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataByNumberAfterSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataNumber>
                    for ListDataByNumberAfterSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataNumber>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_by_number_after(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataByNumberAfterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/GetDataModel" => {
                    #[allow(non_camel_case_types)]
                    struct GetDataModelSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::ModelId>
                    for GetDataModelSvc<T> {
                        type Response = super::DataModelResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModelId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_data_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDataModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ReadDataWithModel" => {
                    #[allow(non_camel_case_types)]
                    struct ReadDataWithModelSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataIdModel>
                    for ReadDataWithModelSvc<T> {
                        type Response = super::DataReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataIdModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_data_with_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadDataWithModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataWithModelByTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataWithModelByTimeSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DataTimeModel>
                    for ListDataWithModelByTimeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataTimeModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_with_model_by_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataWithModelByTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataWithModelByLastTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataWithModelByLastTimeSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DataTimeModel>
                    for ListDataWithModelByLastTimeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataTimeModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_with_model_by_last_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataWithModelByLastTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataWithModelByRangeTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataWithModelByRangeTimeSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DataRangeModel>
                    for ListDataWithModelByRangeTimeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataRangeModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_with_model_by_range_time(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataWithModelByRangeTimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataWithModelByNumberBefore" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataWithModelByNumberBeforeSvc<T: DataService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DataNumberModel>
                    for ListDataWithModelByNumberBeforeSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataNumberModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .list_data_with_model_by_number_before(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataWithModelByNumberBeforeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/ListDataWithModelByNumberAfter" => {
                    #[allow(non_camel_case_types)]
                    struct ListDataWithModelByNumberAfterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DataNumberModel>
                    for ListDataWithModelByNumberAfterSvc<T> {
                        type Response = super::DataListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataNumberModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_data_with_model_by_number_after(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDataWithModelByNumberAfterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/CreateData" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDataSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataSchema>
                    for CreateDataSvc<T> {
                        type Response = super::DataChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/CreateDataWithModel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDataWithModelSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DataSchemaModel>
                    for CreateDataWithModelSvc<T> {
                        type Response = super::DataChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataSchemaModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_data_with_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDataWithModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/DeleteData" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDataSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataId>
                    for DeleteDataSvc<T> {
                        type Response = super::DataChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/data.DataService/DeleteDataWithModel" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDataWithModelSvc<T: DataService>(pub Arc<T>);
                    impl<T: DataService> tonic::server::UnaryService<super::DataIdModel>
                    for DeleteDataWithModelSvc<T> {
                        type Response = super::DataChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DataIdModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_data_with_model(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDataWithModelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: DataService> Clone for DataServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: DataService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataService> tonic::server::NamedService for DataServiceServer<T> {
        const NAME: &'static str = "data.DataService";
    }
}
