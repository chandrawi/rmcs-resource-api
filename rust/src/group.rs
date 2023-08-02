#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupModelSchema {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub models: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeviceSchema {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub devices: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupId {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupName {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCategory {
    #[prost(string, tag = "1")]
    pub category: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupNameCategory {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub category: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupUpdate {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupModel {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub model_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDevice {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub device_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupModelReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<GroupModelSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupModelListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GroupModelSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeviceReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<GroupDeviceSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupDeviceListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<GroupDeviceSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupCreateResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupChangeResponse {}
/// Generated client implementations.
pub mod group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GroupServiceClient<tonic::transport::Channel> {
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
    impl<T> GroupServiceClient<T>
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
        ) -> GroupServiceClient<InterceptedService<T, F>>
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
            GroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn read_group_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelReadResponse>,
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
                "/group.GroupService/ReadGroupModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "ReadGroupModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_model_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupName>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelListResponse>,
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
                "/group.GroupService/ListGroupModelByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "ListGroupModelByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_model_by_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelListResponse>,
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
                "/group.GroupService/ListGroupModelByCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("group.GroupService", "ListGroupModelByCategory"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_model_by_name_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupNameCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelListResponse>,
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
                "/group.GroupService/ListGroupModelByNameCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("group.GroupService", "ListGroupModelByNameCategory"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_group_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupModelSchema>,
        ) -> std::result::Result<
            tonic::Response<super::GroupCreateResponse>,
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
                "/group.GroupService/CreateGroupModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "CreateGroupModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_group_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/UpdateGroupModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "UpdateGroupModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_group_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/DeleteGroupModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "DeleteGroupModel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_group_model_member(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupModel>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/AddGroupModelMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "AddGroupModelMember"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_group_model_member(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupModel>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/RemoveGroupModelMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "RemoveGroupModelMember"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_group_device(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceReadResponse>,
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
                "/group.GroupService/ReadGroupDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "ReadGroupDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_device_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupName>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
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
                "/group.GroupService/ListGroupDeviceByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "ListGroupDeviceByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_device_by_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
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
                "/group.GroupService/ListGroupDeviceByCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("group.GroupService", "ListGroupDeviceByCategory"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_device_by_name_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupNameCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
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
                "/group.GroupService/ListGroupDeviceByNameCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "group.GroupService",
                        "ListGroupDeviceByNameCategory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_group_device(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupDeviceSchema>,
        ) -> std::result::Result<
            tonic::Response<super::GroupCreateResponse>,
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
                "/group.GroupService/CreateGroupDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "CreateGroupDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_group_device(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/UpdateGroupDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "UpdateGroupDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_group_device(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/DeleteGroupDevice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "DeleteGroupDevice"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_group_device_member(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/AddGroupDeviceMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "AddGroupDeviceMember"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_group_device_member(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/RemoveGroupDeviceMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("group.GroupService", "RemoveGroupDeviceMember"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_group_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceReadResponse>,
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
                "/group.GroupService/ReadGroupGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "ReadGroupGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_gateway_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupName>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
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
                "/group.GroupService/ListGroupGatewayByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "ListGroupGatewayByName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_gateway_by_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
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
                "/group.GroupService/ListGroupGatewayByCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("group.GroupService", "ListGroupGatewayByCategory"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_group_gateway_by_name_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupNameCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
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
                "/group.GroupService/ListGroupGatewayByNameCategory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "group.GroupService",
                        "ListGroupGatewayByNameCategory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_group_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupDeviceSchema>,
        ) -> std::result::Result<
            tonic::Response<super::GroupCreateResponse>,
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
                "/group.GroupService/CreateGroupGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "CreateGroupGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_group_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/UpdateGroupGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "UpdateGroupGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_group_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/DeleteGroupGateway",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "DeleteGroupGateway"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_group_gateway_member(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/AddGroupGatewayMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("group.GroupService", "AddGroupGatewayMember"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_group_gateway_member(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
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
                "/group.GroupService/RemoveGroupGatewayMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("group.GroupService", "RemoveGroupGatewayMember"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod group_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GroupServiceServer.
    #[async_trait]
    pub trait GroupService: Send + Sync + 'static {
        async fn read_group_model(
            &self,
            request: tonic::Request<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelReadResponse>,
            tonic::Status,
        >;
        async fn list_group_model_by_name(
            &self,
            request: tonic::Request<super::GroupName>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelListResponse>,
            tonic::Status,
        >;
        async fn list_group_model_by_category(
            &self,
            request: tonic::Request<super::GroupCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelListResponse>,
            tonic::Status,
        >;
        async fn list_group_model_by_name_category(
            &self,
            request: tonic::Request<super::GroupNameCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupModelListResponse>,
            tonic::Status,
        >;
        async fn create_group_model(
            &self,
            request: tonic::Request<super::GroupModelSchema>,
        ) -> std::result::Result<
            tonic::Response<super::GroupCreateResponse>,
            tonic::Status,
        >;
        async fn update_group_model(
            &self,
            request: tonic::Request<super::GroupUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn delete_group_model(
            &self,
            request: tonic::Request<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn add_group_model_member(
            &self,
            request: tonic::Request<super::GroupModel>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn remove_group_model_member(
            &self,
            request: tonic::Request<super::GroupModel>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn read_group_device(
            &self,
            request: tonic::Request<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceReadResponse>,
            tonic::Status,
        >;
        async fn list_group_device_by_name(
            &self,
            request: tonic::Request<super::GroupName>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
            tonic::Status,
        >;
        async fn list_group_device_by_category(
            &self,
            request: tonic::Request<super::GroupCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
            tonic::Status,
        >;
        async fn list_group_device_by_name_category(
            &self,
            request: tonic::Request<super::GroupNameCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
            tonic::Status,
        >;
        async fn create_group_device(
            &self,
            request: tonic::Request<super::GroupDeviceSchema>,
        ) -> std::result::Result<
            tonic::Response<super::GroupCreateResponse>,
            tonic::Status,
        >;
        async fn update_group_device(
            &self,
            request: tonic::Request<super::GroupUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn delete_group_device(
            &self,
            request: tonic::Request<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn add_group_device_member(
            &self,
            request: tonic::Request<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn remove_group_device_member(
            &self,
            request: tonic::Request<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn read_group_gateway(
            &self,
            request: tonic::Request<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceReadResponse>,
            tonic::Status,
        >;
        async fn list_group_gateway_by_name(
            &self,
            request: tonic::Request<super::GroupName>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
            tonic::Status,
        >;
        async fn list_group_gateway_by_category(
            &self,
            request: tonic::Request<super::GroupCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
            tonic::Status,
        >;
        async fn list_group_gateway_by_name_category(
            &self,
            request: tonic::Request<super::GroupNameCategory>,
        ) -> std::result::Result<
            tonic::Response<super::GroupDeviceListResponse>,
            tonic::Status,
        >;
        async fn create_group_gateway(
            &self,
            request: tonic::Request<super::GroupDeviceSchema>,
        ) -> std::result::Result<
            tonic::Response<super::GroupCreateResponse>,
            tonic::Status,
        >;
        async fn update_group_gateway(
            &self,
            request: tonic::Request<super::GroupUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn delete_group_gateway(
            &self,
            request: tonic::Request<super::GroupId>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn add_group_gateway_member(
            &self,
            request: tonic::Request<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
        async fn remove_group_gateway_member(
            &self,
            request: tonic::Request<super::GroupDevice>,
        ) -> std::result::Result<
            tonic::Response<super::GroupChangeResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GroupServiceServer<T: GroupService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GroupService> GroupServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GroupServiceServer<T>
    where
        T: GroupService,
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
                "/group.GroupService/ReadGroupModel" => {
                    #[allow(non_camel_case_types)]
                    struct ReadGroupModelSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupId>
                    for ReadGroupModelSvc<T> {
                        type Response = super::GroupModelReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_group_model(request).await
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
                        let method = ReadGroupModelSvc(inner);
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
                "/group.GroupService/ListGroupModelByName" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupModelByNameSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupName>
                    for ListGroupModelByNameSvc<T> {
                        type Response = super::GroupModelListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_model_by_name(request).await
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
                        let method = ListGroupModelByNameSvc(inner);
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
                "/group.GroupService/ListGroupModelByCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupModelByCategorySvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupCategory>
                    for ListGroupModelByCategorySvc<T> {
                        type Response = super::GroupModelListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupCategory>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_model_by_category(request).await
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
                        let method = ListGroupModelByCategorySvc(inner);
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
                "/group.GroupService/ListGroupModelByNameCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupModelByNameCategorySvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupNameCategory>
                    for ListGroupModelByNameCategorySvc<T> {
                        type Response = super::GroupModelListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupNameCategory>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_model_by_name_category(request).await
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
                        let method = ListGroupModelByNameCategorySvc(inner);
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
                "/group.GroupService/CreateGroupModel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupModelSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupModelSchema>
                    for CreateGroupModelSvc<T> {
                        type Response = super::GroupCreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupModelSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_group_model(request).await
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
                        let method = CreateGroupModelSvc(inner);
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
                "/group.GroupService/UpdateGroupModel" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupModelSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupUpdate>
                    for UpdateGroupModelSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_model(request).await
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
                        let method = UpdateGroupModelSvc(inner);
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
                "/group.GroupService/DeleteGroupModel" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGroupModelSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupId>
                    for DeleteGroupModelSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_group_model(request).await
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
                        let method = DeleteGroupModelSvc(inner);
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
                "/group.GroupService/AddGroupModelMember" => {
                    #[allow(non_camel_case_types)]
                    struct AddGroupModelMemberSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupModel>
                    for AddGroupModelMemberSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_group_model_member(request).await
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
                        let method = AddGroupModelMemberSvc(inner);
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
                "/group.GroupService/RemoveGroupModelMember" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGroupModelMemberSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupModel>
                    for RemoveGroupModelMemberSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupModel>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_group_model_member(request).await
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
                        let method = RemoveGroupModelMemberSvc(inner);
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
                "/group.GroupService/ReadGroupDevice" => {
                    #[allow(non_camel_case_types)]
                    struct ReadGroupDeviceSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupId>
                    for ReadGroupDeviceSvc<T> {
                        type Response = super::GroupDeviceReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_group_device(request).await
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
                        let method = ReadGroupDeviceSvc(inner);
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
                "/group.GroupService/ListGroupDeviceByName" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupDeviceByNameSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupName>
                    for ListGroupDeviceByNameSvc<T> {
                        type Response = super::GroupDeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_device_by_name(request).await
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
                        let method = ListGroupDeviceByNameSvc(inner);
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
                "/group.GroupService/ListGroupDeviceByCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupDeviceByCategorySvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupCategory>
                    for ListGroupDeviceByCategorySvc<T> {
                        type Response = super::GroupDeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupCategory>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_device_by_category(request).await
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
                        let method = ListGroupDeviceByCategorySvc(inner);
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
                "/group.GroupService/ListGroupDeviceByNameCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupDeviceByNameCategorySvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupNameCategory>
                    for ListGroupDeviceByNameCategorySvc<T> {
                        type Response = super::GroupDeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupNameCategory>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_device_by_name_category(request).await
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
                        let method = ListGroupDeviceByNameCategorySvc(inner);
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
                "/group.GroupService/CreateGroupDevice" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupDeviceSvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupDeviceSchema>
                    for CreateGroupDeviceSvc<T> {
                        type Response = super::GroupCreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupDeviceSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_group_device(request).await
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
                        let method = CreateGroupDeviceSvc(inner);
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
                "/group.GroupService/UpdateGroupDevice" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupDeviceSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupUpdate>
                    for UpdateGroupDeviceSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_device(request).await
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
                        let method = UpdateGroupDeviceSvc(inner);
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
                "/group.GroupService/DeleteGroupDevice" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGroupDeviceSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupId>
                    for DeleteGroupDeviceSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_group_device(request).await
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
                        let method = DeleteGroupDeviceSvc(inner);
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
                "/group.GroupService/AddGroupDeviceMember" => {
                    #[allow(non_camel_case_types)]
                    struct AddGroupDeviceMemberSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupDevice>
                    for AddGroupDeviceMemberSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupDevice>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_group_device_member(request).await
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
                        let method = AddGroupDeviceMemberSvc(inner);
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
                "/group.GroupService/RemoveGroupDeviceMember" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGroupDeviceMemberSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupDevice>
                    for RemoveGroupDeviceMemberSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupDevice>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_group_device_member(request).await
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
                        let method = RemoveGroupDeviceMemberSvc(inner);
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
                "/group.GroupService/ReadGroupGateway" => {
                    #[allow(non_camel_case_types)]
                    struct ReadGroupGatewaySvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupId>
                    for ReadGroupGatewaySvc<T> {
                        type Response = super::GroupDeviceReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_group_gateway(request).await
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
                        let method = ReadGroupGatewaySvc(inner);
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
                "/group.GroupService/ListGroupGatewayByName" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupGatewayByNameSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupName>
                    for ListGroupGatewayByNameSvc<T> {
                        type Response = super::GroupDeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupName>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_gateway_by_name(request).await
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
                        let method = ListGroupGatewayByNameSvc(inner);
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
                "/group.GroupService/ListGroupGatewayByCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupGatewayByCategorySvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupCategory>
                    for ListGroupGatewayByCategorySvc<T> {
                        type Response = super::GroupDeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupCategory>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_gateway_by_category(request).await
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
                        let method = ListGroupGatewayByCategorySvc(inner);
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
                "/group.GroupService/ListGroupGatewayByNameCategory" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupGatewayByNameCategorySvc<T: GroupService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupNameCategory>
                    for ListGroupGatewayByNameCategorySvc<T> {
                        type Response = super::GroupDeviceListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupNameCategory>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_group_gateway_by_name_category(request).await
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
                        let method = ListGroupGatewayByNameCategorySvc(inner);
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
                "/group.GroupService/CreateGroupGateway" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupGatewaySvc<T: GroupService>(pub Arc<T>);
                    impl<
                        T: GroupService,
                    > tonic::server::UnaryService<super::GroupDeviceSchema>
                    for CreateGroupGatewaySvc<T> {
                        type Response = super::GroupCreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupDeviceSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_group_gateway(request).await
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
                        let method = CreateGroupGatewaySvc(inner);
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
                "/group.GroupService/UpdateGroupGateway" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupGatewaySvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupUpdate>
                    for UpdateGroupGatewaySvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_group_gateway(request).await
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
                        let method = UpdateGroupGatewaySvc(inner);
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
                "/group.GroupService/DeleteGroupGateway" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGroupGatewaySvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupId>
                    for DeleteGroupGatewaySvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_group_gateway(request).await
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
                        let method = DeleteGroupGatewaySvc(inner);
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
                "/group.GroupService/AddGroupGatewayMember" => {
                    #[allow(non_camel_case_types)]
                    struct AddGroupGatewayMemberSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupDevice>
                    for AddGroupGatewayMemberSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupDevice>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_group_gateway_member(request).await
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
                        let method = AddGroupGatewayMemberSvc(inner);
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
                "/group.GroupService/RemoveGroupGatewayMember" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGroupGatewayMemberSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GroupDevice>
                    for RemoveGroupGatewayMemberSvc<T> {
                        type Response = super::GroupChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GroupDevice>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_group_gateway_member(request).await
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
                        let method = RemoveGroupGatewayMemberSvc(inner);
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
    impl<T: GroupService> Clone for GroupServiceServer<T> {
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
    impl<T: GroupService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GroupService> tonic::server::NamedService for GroupServiceServer<T> {
        const NAME: &'static str = "group.GroupService";
    }
}
