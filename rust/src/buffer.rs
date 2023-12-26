#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferSchema {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub device_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub model_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    #[prost(bytes = "vec", tag = "5")]
    pub data_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "super::common::DataType", repeated, tag = "6")]
    pub data_type: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "BufferStatus", tag = "7")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferId {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferTime {
    #[prost(bytes = "vec", tag = "1")]
    pub device_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub model_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    #[prost(enumeration = "BufferStatus", optional, tag = "4")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferSelector {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub device_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub model_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "BufferStatus", optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuffersSelector {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub device_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub model_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "BufferStatus", optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
    #[prost(uint32, tag = "4")]
    pub number: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferUpdate {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub data_bytes: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "super::common::DataType", repeated, tag = "7")]
    pub data_type: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "BufferStatus", optional, tag = "8")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<BufferSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<BufferSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferCreateResponse {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferChangeResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BufferStatus {
    Default = 0,
    Error = 1,
    Delete = 2,
    Hold = 3,
    SendUplink = 4,
    SendDownlink = 5,
    TransferLocal = 6,
    TransferGateway = 7,
    TransferServer = 8,
    Backup = 9,
    Restore = 10,
    Analysis1 = 11,
    Analysis2 = 12,
    Analysis3 = 13,
    Analysis4 = 14,
    Analysis5 = 15,
    Analysis6 = 16,
    Analysis7 = 17,
    Analysis8 = 18,
    Analysis9 = 19,
    Analysis10 = 20,
}
impl BufferStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BufferStatus::Default => "DEFAULT",
            BufferStatus::Error => "ERROR",
            BufferStatus::Delete => "DELETE",
            BufferStatus::Hold => "HOLD",
            BufferStatus::SendUplink => "SEND_UPLINK",
            BufferStatus::SendDownlink => "SEND_DOWNLINK",
            BufferStatus::TransferLocal => "TRANSFER_LOCAL",
            BufferStatus::TransferGateway => "TRANSFER_GATEWAY",
            BufferStatus::TransferServer => "TRANSFER_SERVER",
            BufferStatus::Backup => "BACKUP",
            BufferStatus::Restore => "RESTORE",
            BufferStatus::Analysis1 => "ANALYSIS_1",
            BufferStatus::Analysis2 => "ANALYSIS_2",
            BufferStatus::Analysis3 => "ANALYSIS_3",
            BufferStatus::Analysis4 => "ANALYSIS_4",
            BufferStatus::Analysis5 => "ANALYSIS_5",
            BufferStatus::Analysis6 => "ANALYSIS_6",
            BufferStatus::Analysis7 => "ANALYSIS_7",
            BufferStatus::Analysis8 => "ANALYSIS_8",
            BufferStatus::Analysis9 => "ANALYSIS_9",
            BufferStatus::Analysis10 => "ANALYSIS_10",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "ERROR" => Some(Self::Error),
            "DELETE" => Some(Self::Delete),
            "HOLD" => Some(Self::Hold),
            "SEND_UPLINK" => Some(Self::SendUplink),
            "SEND_DOWNLINK" => Some(Self::SendDownlink),
            "TRANSFER_LOCAL" => Some(Self::TransferLocal),
            "TRANSFER_GATEWAY" => Some(Self::TransferGateway),
            "TRANSFER_SERVER" => Some(Self::TransferServer),
            "BACKUP" => Some(Self::Backup),
            "RESTORE" => Some(Self::Restore),
            "ANALYSIS_1" => Some(Self::Analysis1),
            "ANALYSIS_2" => Some(Self::Analysis2),
            "ANALYSIS_3" => Some(Self::Analysis3),
            "ANALYSIS_4" => Some(Self::Analysis4),
            "ANALYSIS_5" => Some(Self::Analysis5),
            "ANALYSIS_6" => Some(Self::Analysis6),
            "ANALYSIS_7" => Some(Self::Analysis7),
            "ANALYSIS_8" => Some(Self::Analysis8),
            "ANALYSIS_9" => Some(Self::Analysis9),
            "ANALYSIS_10" => Some(Self::Analysis10),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod buffer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BufferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BufferServiceClient<tonic::transport::Channel> {
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
    impl<T> BufferServiceClient<T>
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
        ) -> BufferServiceClient<InterceptedService<T, F>>
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
            BufferServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn read_buffer(
            &mut self,
            request: impl tonic::IntoRequest<super::BufferId>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
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
                "/buffer.BufferService/ReadBuffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "ReadBuffer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_buffer_by_time(
            &mut self,
            request: impl tonic::IntoRequest<super::BufferTime>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
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
                "/buffer.BufferService/ReadBufferByTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "ReadBufferByTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_buffer_first(
            &mut self,
            request: impl tonic::IntoRequest<super::BufferSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
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
                "/buffer.BufferService/ReadBufferFirst",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "ReadBufferFirst"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_buffer_last(
            &mut self,
            request: impl tonic::IntoRequest<super::BufferSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
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
                "/buffer.BufferService/ReadBufferLast",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "ReadBufferLast"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_buffer_first(
            &mut self,
            request: impl tonic::IntoRequest<super::BuffersSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferListResponse>,
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
                "/buffer.BufferService/ListBufferFirst",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "ListBufferFirst"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_buffer_last(
            &mut self,
            request: impl tonic::IntoRequest<super::BuffersSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferListResponse>,
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
                "/buffer.BufferService/ListBufferLast",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "ListBufferLast"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_buffer(
            &mut self,
            request: impl tonic::IntoRequest<super::BufferSchema>,
        ) -> std::result::Result<
            tonic::Response<super::BufferCreateResponse>,
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
                "/buffer.BufferService/CreateBuffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "CreateBuffer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_buffer(
            &mut self,
            request: impl tonic::IntoRequest<super::BufferUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::BufferChangeResponse>,
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
                "/buffer.BufferService/UpdateBuffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "UpdateBuffer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_buffer(
            &mut self,
            request: impl tonic::IntoRequest<super::BufferId>,
        ) -> std::result::Result<
            tonic::Response<super::BufferChangeResponse>,
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
                "/buffer.BufferService/DeleteBuffer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("buffer.BufferService", "DeleteBuffer"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod buffer_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BufferServiceServer.
    #[async_trait]
    pub trait BufferService: Send + Sync + 'static {
        async fn read_buffer(
            &self,
            request: tonic::Request<super::BufferId>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
            tonic::Status,
        >;
        async fn read_buffer_by_time(
            &self,
            request: tonic::Request<super::BufferTime>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
            tonic::Status,
        >;
        async fn read_buffer_first(
            &self,
            request: tonic::Request<super::BufferSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
            tonic::Status,
        >;
        async fn read_buffer_last(
            &self,
            request: tonic::Request<super::BufferSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferReadResponse>,
            tonic::Status,
        >;
        async fn list_buffer_first(
            &self,
            request: tonic::Request<super::BuffersSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferListResponse>,
            tonic::Status,
        >;
        async fn list_buffer_last(
            &self,
            request: tonic::Request<super::BuffersSelector>,
        ) -> std::result::Result<
            tonic::Response<super::BufferListResponse>,
            tonic::Status,
        >;
        async fn create_buffer(
            &self,
            request: tonic::Request<super::BufferSchema>,
        ) -> std::result::Result<
            tonic::Response<super::BufferCreateResponse>,
            tonic::Status,
        >;
        async fn update_buffer(
            &self,
            request: tonic::Request<super::BufferUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::BufferChangeResponse>,
            tonic::Status,
        >;
        async fn delete_buffer(
            &self,
            request: tonic::Request<super::BufferId>,
        ) -> std::result::Result<
            tonic::Response<super::BufferChangeResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BufferServiceServer<T: BufferService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BufferService> BufferServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BufferServiceServer<T>
    where
        T: BufferService,
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
                "/buffer.BufferService/ReadBuffer" => {
                    #[allow(non_camel_case_types)]
                    struct ReadBufferSvc<T: BufferService>(pub Arc<T>);
                    impl<T: BufferService> tonic::server::UnaryService<super::BufferId>
                    for ReadBufferSvc<T> {
                        type Response = super::BufferReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BufferId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).read_buffer(request).await };
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
                        let method = ReadBufferSvc(inner);
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
                "/buffer.BufferService/ReadBufferByTime" => {
                    #[allow(non_camel_case_types)]
                    struct ReadBufferByTimeSvc<T: BufferService>(pub Arc<T>);
                    impl<T: BufferService> tonic::server::UnaryService<super::BufferTime>
                    for ReadBufferByTimeSvc<T> {
                        type Response = super::BufferReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BufferTime>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_buffer_by_time(request).await
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
                        let method = ReadBufferByTimeSvc(inner);
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
                "/buffer.BufferService/ReadBufferFirst" => {
                    #[allow(non_camel_case_types)]
                    struct ReadBufferFirstSvc<T: BufferService>(pub Arc<T>);
                    impl<
                        T: BufferService,
                    > tonic::server::UnaryService<super::BufferSelector>
                    for ReadBufferFirstSvc<T> {
                        type Response = super::BufferReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BufferSelector>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_buffer_first(request).await
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
                        let method = ReadBufferFirstSvc(inner);
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
                "/buffer.BufferService/ReadBufferLast" => {
                    #[allow(non_camel_case_types)]
                    struct ReadBufferLastSvc<T: BufferService>(pub Arc<T>);
                    impl<
                        T: BufferService,
                    > tonic::server::UnaryService<super::BufferSelector>
                    for ReadBufferLastSvc<T> {
                        type Response = super::BufferReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BufferSelector>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).read_buffer_last(request).await
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
                        let method = ReadBufferLastSvc(inner);
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
                "/buffer.BufferService/ListBufferFirst" => {
                    #[allow(non_camel_case_types)]
                    struct ListBufferFirstSvc<T: BufferService>(pub Arc<T>);
                    impl<
                        T: BufferService,
                    > tonic::server::UnaryService<super::BuffersSelector>
                    for ListBufferFirstSvc<T> {
                        type Response = super::BufferListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BuffersSelector>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_buffer_first(request).await
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
                        let method = ListBufferFirstSvc(inner);
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
                "/buffer.BufferService/ListBufferLast" => {
                    #[allow(non_camel_case_types)]
                    struct ListBufferLastSvc<T: BufferService>(pub Arc<T>);
                    impl<
                        T: BufferService,
                    > tonic::server::UnaryService<super::BuffersSelector>
                    for ListBufferLastSvc<T> {
                        type Response = super::BufferListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BuffersSelector>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_buffer_last(request).await
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
                        let method = ListBufferLastSvc(inner);
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
                "/buffer.BufferService/CreateBuffer" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBufferSvc<T: BufferService>(pub Arc<T>);
                    impl<
                        T: BufferService,
                    > tonic::server::UnaryService<super::BufferSchema>
                    for CreateBufferSvc<T> {
                        type Response = super::BufferCreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BufferSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_buffer(request).await
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
                        let method = CreateBufferSvc(inner);
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
                "/buffer.BufferService/UpdateBuffer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBufferSvc<T: BufferService>(pub Arc<T>);
                    impl<
                        T: BufferService,
                    > tonic::server::UnaryService<super::BufferUpdate>
                    for UpdateBufferSvc<T> {
                        type Response = super::BufferChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BufferUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_buffer(request).await
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
                        let method = UpdateBufferSvc(inner);
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
                "/buffer.BufferService/DeleteBuffer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBufferSvc<T: BufferService>(pub Arc<T>);
                    impl<T: BufferService> tonic::server::UnaryService<super::BufferId>
                    for DeleteBufferSvc<T> {
                        type Response = super::BufferChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BufferId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_buffer(request).await
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
                        let method = DeleteBufferSvc(inner);
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
    impl<T: BufferService> Clone for BufferServiceServer<T> {
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
    impl<T: BufferService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BufferService> tonic::server::NamedService for BufferServiceServer<T> {
        const NAME: &'static str = "buffer.BufferService";
    }
}
