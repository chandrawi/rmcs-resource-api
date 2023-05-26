#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSchema {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub gateway_id: u64,
    #[prost(string, tag = "3")]
    pub serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub device_type: ::core::option::Option<TypeSchema>,
    #[prost(message, repeated, tag = "7")]
    pub configs: ::prost::alloc::vec::Vec<ConfigSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewaySchema {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub serial_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub gateway_type: ::core::option::Option<TypeSchema>,
    #[prost(message, repeated, tag = "6")]
    pub configs: ::prost::alloc::vec::Vec<ConfigSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceId {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayId {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerialNumber {
    #[prost(string, tag = "1")]
    pub serial_number: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceName {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayName {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceGatewayType {
    #[prost(uint64, tag = "1")]
    pub gateway_id: u64,
    #[prost(uint32, tag = "2")]
    pub type_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceGatewayName {
    #[prost(uint64, tag = "1")]
    pub gateway_id: u64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceUpdate {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, optional, tag = "2")]
    pub gateway_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub serial_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub type_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayUpdate {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, optional, tag = "2")]
    pub serial_number: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub type_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSchema {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(uint64, tag = "2")]
    pub device_id: u64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub config_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "super::common::ConfigType", tag = "5")]
    pub config_type: i32,
    #[prost(string, tag = "6")]
    pub category: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigId {
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigUpdate {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub config_bytes: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "super::common::ConfigType", optional, tag = "4")]
    pub config_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeSchema {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, repeated, tag = "4")]
    pub models: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeId {
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeName {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeUpdate {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeModel {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(uint32, tag = "2")]
    pub model_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<DeviceSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<DeviceSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceChangeResponse {
    #[prost(enumeration = "super::common::ResponseStatus", tag = "1")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<GatewaySchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GatewaySchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayChangeResponse {
    #[prost(enumeration = "super::common::ResponseStatus", tag = "1")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ConfigSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<ConfigSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigCreateResponse {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigChangeResponse {
    #[prost(enumeration = "super::common::ResponseStatus", tag = "1")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<TypeSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<TypeSchema>,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeCreateResponse {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(enumeration = "super::common::ResponseStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeChangeResponse {
    #[prost(enumeration = "super::common::ResponseStatus", tag = "1")]
    pub status: i32,
}
/// Generated client implementations.
pub mod device_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DeviceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceServiceClient<tonic::transport::Channel> {
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
    impl<T> DeviceServiceClient<T>
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
        ) -> DeviceServiceClient<InterceptedService<T, F>>
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
            DeviceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn read_device(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceReadResponse>,
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
                "/device.DeviceService/ReadDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ReadDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_device_by_sn(
            &mut self,
            request: impl tonic::IntoRequest<super::SerialNumber>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceReadResponse>,
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
                "/device.DeviceService/ReadDeviceBySn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ReadDeviceBySn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_device_by_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewayId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
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
                "/device.DeviceService/ListDeviceByGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ListDeviceByGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_device_by_type(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
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
                "/device.DeviceService/ListDeviceByType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ListDeviceByType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_device_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceName>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
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
                "/device.DeviceService/ListDeviceByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ListDeviceByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_device_by_gateway_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceGatewayType>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
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
                "/device.DeviceService/ListDeviceByGatewayType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("device.DeviceService", "ListDeviceByGatewayType"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_device_by_gateway_name(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceGatewayName>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
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
                "/device.DeviceService/ListDeviceByGatewayName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("device.DeviceService", "ListDeviceByGatewayName"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_device(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceSchema>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceChangeResponse>,
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
                "/device.DeviceService/CreateDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "CreateDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_device(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceChangeResponse>,
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
                "/device.DeviceService/UpdateDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "UpdateDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_device(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceChangeResponse>,
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
                "/device.DeviceService/DeleteDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "DeleteDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewayId>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayReadResponse>,
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
                "/device.DeviceService/ReadGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ReadGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_gateway_by_sn(
            &mut self,
            request: impl tonic::IntoRequest<super::SerialNumber>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayReadResponse>,
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
                "/device.DeviceService/ReadGatewayBySn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ReadGatewayBySn"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_gateway_by_type(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayListResponse>,
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
                "/device.DeviceService/ListGatewayByType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ListGatewayByType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_gateway_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewayName>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayListResponse>,
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
                "/device.DeviceService/ListGatewayByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ListGatewayByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewaySchema>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayChangeResponse>,
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
                "/device.DeviceService/CreateGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "CreateGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewayUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayChangeResponse>,
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
                "/device.DeviceService/UpdateGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "UpdateGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GatewayId>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayChangeResponse>,
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
                "/device.DeviceService/DeleteGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "DeleteGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_device_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigId>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigReadResponse>,
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
                "/device.DeviceService/ReadDeviceConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ReadDeviceConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_device_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeviceId>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigListResponse>,
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
                "/device.DeviceService/ListDeviceConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ListDeviceConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_device_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigSchema>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigCreateResponse>,
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
                "/device.DeviceService/CreateDeviceConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "CreateDeviceConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_device_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigChangeResponse>,
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
                "/device.DeviceService/UpdateDeviceConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "UpdateDeviceConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_device_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigId>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigChangeResponse>,
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
                "/device.DeviceService/DeleteDeviceConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "DeleteDeviceConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_type(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::TypeReadResponse>,
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
                "/device.DeviceService/ReadType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ReadType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_type_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeName>,
        ) -> std::result::Result<
            tonic::Response<super::TypeListResponse>,
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
                "/device.DeviceService/ListTypeByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "ListTypeByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_type(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeSchema>,
        ) -> std::result::Result<
            tonic::Response<super::TypeCreateResponse>,
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
                "/device.DeviceService/CreateType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "CreateType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_type(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
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
                "/device.DeviceService/UpdateType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "UpdateType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_type(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
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
                "/device.DeviceService/DeleteType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "DeleteType"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_type_model(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeModel>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
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
                "/device.DeviceService/AddTypeModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "AddTypeModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_type_model(
            &mut self,
            request: impl tonic::IntoRequest<super::TypeModel>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
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
                "/device.DeviceService/RemoveTypeModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("device.DeviceService", "RemoveTypeModel"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod device_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DeviceServiceServer.
    #[async_trait]
    pub trait DeviceService: Send + Sync + 'static {
        async fn read_device(
            &self,
            request: tonic::Request<super::DeviceId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceReadResponse>,
            tonic::Status,
        >;
        async fn read_device_by_sn(
            &self,
            request: tonic::Request<super::SerialNumber>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceReadResponse>,
            tonic::Status,
        >;
        async fn list_device_by_gateway(
            &self,
            request: tonic::Request<super::GatewayId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
            tonic::Status,
        >;
        async fn list_device_by_type(
            &self,
            request: tonic::Request<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
            tonic::Status,
        >;
        async fn list_device_by_name(
            &self,
            request: tonic::Request<super::DeviceName>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
            tonic::Status,
        >;
        async fn list_device_by_gateway_type(
            &self,
            request: tonic::Request<super::DeviceGatewayType>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
            tonic::Status,
        >;
        async fn list_device_by_gateway_name(
            &self,
            request: tonic::Request<super::DeviceGatewayName>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceListResponse>,
            tonic::Status,
        >;
        async fn create_device(
            &self,
            request: tonic::Request<super::DeviceSchema>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceChangeResponse>,
            tonic::Status,
        >;
        async fn update_device(
            &self,
            request: tonic::Request<super::DeviceUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceChangeResponse>,
            tonic::Status,
        >;
        async fn delete_device(
            &self,
            request: tonic::Request<super::DeviceId>,
        ) -> std::result::Result<
            tonic::Response<super::DeviceChangeResponse>,
            tonic::Status,
        >;
        async fn read_gateway(
            &self,
            request: tonic::Request<super::GatewayId>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayReadResponse>,
            tonic::Status,
        >;
        async fn read_gateway_by_sn(
            &self,
            request: tonic::Request<super::SerialNumber>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayReadResponse>,
            tonic::Status,
        >;
        async fn list_gateway_by_type(
            &self,
            request: tonic::Request<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayListResponse>,
            tonic::Status,
        >;
        async fn list_gateway_by_name(
            &self,
            request: tonic::Request<super::GatewayName>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayListResponse>,
            tonic::Status,
        >;
        async fn create_gateway(
            &self,
            request: tonic::Request<super::GatewaySchema>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayChangeResponse>,
            tonic::Status,
        >;
        async fn update_gateway(
            &self,
            request: tonic::Request<super::GatewayUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayChangeResponse>,
            tonic::Status,
        >;
        async fn delete_gateway(
            &self,
            request: tonic::Request<super::GatewayId>,
        ) -> std::result::Result<
            tonic::Response<super::GatewayChangeResponse>,
            tonic::Status,
        >;
        async fn read_device_config(
            &self,
            request: tonic::Request<super::ConfigId>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigReadResponse>,
            tonic::Status,
        >;
        async fn list_device_config(
            &self,
            request: tonic::Request<super::DeviceId>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigListResponse>,
            tonic::Status,
        >;
        async fn create_device_config(
            &self,
            request: tonic::Request<super::ConfigSchema>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigCreateResponse>,
            tonic::Status,
        >;
        async fn update_device_config(
            &self,
            request: tonic::Request<super::ConfigUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigChangeResponse>,
            tonic::Status,
        >;
        async fn delete_device_config(
            &self,
            request: tonic::Request<super::ConfigId>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigChangeResponse>,
            tonic::Status,
        >;
        async fn read_type(
            &self,
            request: tonic::Request<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::TypeReadResponse>,
            tonic::Status,
        >;
        async fn list_type_by_name(
            &self,
            request: tonic::Request<super::TypeName>,
        ) -> std::result::Result<
            tonic::Response<super::TypeListResponse>,
            tonic::Status,
        >;
        async fn create_type(
            &self,
            request: tonic::Request<super::TypeSchema>,
        ) -> std::result::Result<
            tonic::Response<super::TypeCreateResponse>,
            tonic::Status,
        >;
        async fn update_type(
            &self,
            request: tonic::Request<super::TypeUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
            tonic::Status,
        >;
        async fn delete_type(
            &self,
            request: tonic::Request<super::TypeId>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
            tonic::Status,
        >;
        async fn add_type_model(
            &self,
            request: tonic::Request<super::TypeModel>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
            tonic::Status,
        >;
        async fn remove_type_model(
            &self,
            request: tonic::Request<super::TypeModel>,
        ) -> std::result::Result<
            tonic::Response<super::TypeChangeResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DeviceServiceServer<T: DeviceService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DeviceService> DeviceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DeviceServiceServer<T>
    where
        T: DeviceService,
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
                "/device.DeviceService/ReadDevice" => {
                    #[allow(non_camel_case_types)]
                    struct ReadDeviceSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::DeviceId>
                    for ReadDeviceSvc<T> {
                        type Response = super::DeviceReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).read_device(request).await };
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
                        let method = ReadDeviceSvc(inner);
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
                "/device.DeviceService/ReadDeviceBySn" => {
                    #[allow(non_camel_case_types)]
                    struct ReadDeviceBySnSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::SerialNumber>
                    for ReadDeviceBySnSvc<T> {
                        type Response = super::DeviceReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SerialNumber>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_device_by_sn(request).await
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
                        let method = ReadDeviceBySnSvc(inner);
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
                "/device.DeviceService/ListDeviceByGateway" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceByGatewaySvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::GatewayId>
                    for ListDeviceByGatewaySvc<T> {
                        type Response = super::DeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewayId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_device_by_gateway(request).await
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
                        let method = ListDeviceByGatewaySvc(inner);
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
                "/device.DeviceService/ListDeviceByType" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceByTypeSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeId>
                    for ListDeviceByTypeSvc<T> {
                        type Response = super::DeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_device_by_type(request).await
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
                        let method = ListDeviceByTypeSvc(inner);
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
                "/device.DeviceService/ListDeviceByName" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceByNameSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::DeviceName>
                    for ListDeviceByNameSvc<T> {
                        type Response = super::DeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_device_by_name(request).await
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
                        let method = ListDeviceByNameSvc(inner);
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
                "/device.DeviceService/ListDeviceByGatewayType" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceByGatewayTypeSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DeviceGatewayType>
                    for ListDeviceByGatewayTypeSvc<T> {
                        type Response = super::DeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceGatewayType>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_device_by_gateway_type(request).await
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
                        let method = ListDeviceByGatewayTypeSvc(inner);
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
                "/device.DeviceService/ListDeviceByGatewayName" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceByGatewayNameSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DeviceGatewayName>
                    for ListDeviceByGatewayNameSvc<T> {
                        type Response = super::DeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceGatewayName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_device_by_gateway_name(request).await
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
                        let method = ListDeviceByGatewayNameSvc(inner);
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
                "/device.DeviceService/CreateDevice" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDeviceSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DeviceSchema>
                    for CreateDeviceSvc<T> {
                        type Response = super::DeviceChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_device(request).await
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
                        let method = CreateDeviceSvc(inner);
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
                "/device.DeviceService/UpdateDevice" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDeviceSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::DeviceUpdate>
                    for UpdateDeviceSvc<T> {
                        type Response = super::DeviceChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_device(request).await
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
                        let method = UpdateDeviceSvc(inner);
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
                "/device.DeviceService/DeleteDevice" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDeviceSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::DeviceId>
                    for DeleteDeviceSvc<T> {
                        type Response = super::DeviceChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_device(request).await
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
                        let method = DeleteDeviceSvc(inner);
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
                "/device.DeviceService/ReadGateway" => {
                    #[allow(non_camel_case_types)]
                    struct ReadGatewaySvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::GatewayId>
                    for ReadGatewaySvc<T> {
                        type Response = super::GatewayReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewayId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_gateway(request).await
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
                        let method = ReadGatewaySvc(inner);
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
                "/device.DeviceService/ReadGatewayBySn" => {
                    #[allow(non_camel_case_types)]
                    struct ReadGatewayBySnSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::SerialNumber>
                    for ReadGatewayBySnSvc<T> {
                        type Response = super::GatewayReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SerialNumber>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_gateway_by_sn(request).await
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
                        let method = ReadGatewayBySnSvc(inner);
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
                "/device.DeviceService/ListGatewayByType" => {
                    #[allow(non_camel_case_types)]
                    struct ListGatewayByTypeSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeId>
                    for ListGatewayByTypeSvc<T> {
                        type Response = super::GatewayListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_gateway_by_type(request).await
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
                        let method = ListGatewayByTypeSvc(inner);
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
                "/device.DeviceService/ListGatewayByName" => {
                    #[allow(non_camel_case_types)]
                    struct ListGatewayByNameSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::GatewayName>
                    for ListGatewayByNameSvc<T> {
                        type Response = super::GatewayListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewayName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_gateway_by_name(request).await
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
                        let method = ListGatewayByNameSvc(inner);
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
                "/device.DeviceService/CreateGateway" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGatewaySvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::GatewaySchema>
                    for CreateGatewaySvc<T> {
                        type Response = super::GatewayChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewaySchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_gateway(request).await
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
                        let method = CreateGatewaySvc(inner);
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
                "/device.DeviceService/UpdateGateway" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGatewaySvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::GatewayUpdate>
                    for UpdateGatewaySvc<T> {
                        type Response = super::GatewayChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewayUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_gateway(request).await
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
                        let method = UpdateGatewaySvc(inner);
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
                "/device.DeviceService/DeleteGateway" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGatewaySvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::GatewayId>
                    for DeleteGatewaySvc<T> {
                        type Response = super::GatewayChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GatewayId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_gateway(request).await
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
                        let method = DeleteGatewaySvc(inner);
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
                "/device.DeviceService/ReadDeviceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ReadDeviceConfigSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::ConfigId>
                    for ReadDeviceConfigSvc<T> {
                        type Response = super::ConfigReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_device_config(request).await
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
                        let method = ReadDeviceConfigSvc(inner);
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
                "/device.DeviceService/ListDeviceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeviceConfigSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::DeviceId>
                    for ListDeviceConfigSvc<T> {
                        type Response = super::ConfigListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeviceId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_device_config(request).await
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
                        let method = ListDeviceConfigSvc(inner);
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
                "/device.DeviceService/CreateDeviceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDeviceConfigSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::ConfigSchema>
                    for CreateDeviceConfigSvc<T> {
                        type Response = super::ConfigCreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_device_config(request).await
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
                        let method = CreateDeviceConfigSvc(inner);
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
                "/device.DeviceService/UpdateDeviceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDeviceConfigSvc<T: DeviceService>(pub Arc<T>);
                    impl<
                        T: DeviceService,
                    > tonic::server::UnaryService<super::ConfigUpdate>
                    for UpdateDeviceConfigSvc<T> {
                        type Response = super::ConfigChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_device_config(request).await
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
                        let method = UpdateDeviceConfigSvc(inner);
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
                "/device.DeviceService/DeleteDeviceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDeviceConfigSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::ConfigId>
                    for DeleteDeviceConfigSvc<T> {
                        type Response = super::ConfigChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_device_config(request).await
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
                        let method = DeleteDeviceConfigSvc(inner);
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
                "/device.DeviceService/ReadType" => {
                    #[allow(non_camel_case_types)]
                    struct ReadTypeSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeId>
                    for ReadTypeSvc<T> {
                        type Response = super::TypeReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).read_type(request).await };
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
                        let method = ReadTypeSvc(inner);
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
                "/device.DeviceService/ListTypeByName" => {
                    #[allow(non_camel_case_types)]
                    struct ListTypeByNameSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeName>
                    for ListTypeByNameSvc<T> {
                        type Response = super::TypeListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_type_by_name(request).await
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
                        let method = ListTypeByNameSvc(inner);
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
                "/device.DeviceService/CreateType" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTypeSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeSchema>
                    for CreateTypeSvc<T> {
                        type Response = super::TypeCreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_type(request).await };
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
                        let method = CreateTypeSvc(inner);
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
                "/device.DeviceService/UpdateType" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTypeSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeUpdate>
                    for UpdateTypeSvc<T> {
                        type Response = super::TypeChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_type(request).await };
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
                        let method = UpdateTypeSvc(inner);
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
                "/device.DeviceService/DeleteType" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTypeSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeId>
                    for DeleteTypeSvc<T> {
                        type Response = super::TypeChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_type(request).await };
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
                        let method = DeleteTypeSvc(inner);
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
                "/device.DeviceService/AddTypeModel" => {
                    #[allow(non_camel_case_types)]
                    struct AddTypeModelSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeModel>
                    for AddTypeModelSvc<T> {
                        type Response = super::TypeChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_type_model(request).await
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
                        let method = AddTypeModelSvc(inner);
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
                "/device.DeviceService/RemoveTypeModel" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTypeModelSvc<T: DeviceService>(pub Arc<T>);
                    impl<T: DeviceService> tonic::server::UnaryService<super::TypeModel>
                    for RemoveTypeModelSvc<T> {
                        type Response = super::TypeChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TypeModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_type_model(request).await
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
                        let method = RemoveTypeModelSvc(inner);
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
    impl<T: DeviceService> Clone for DeviceServiceServer<T> {
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
    impl<T: DeviceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DeviceService> tonic::server::NamedService for DeviceServiceServer<T> {
        const NAME: &'static str = "device.DeviceService";
    }
}
