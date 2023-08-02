#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSchema {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub device_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "LogStatus", tag = "3")]
    pub status: i32,
    #[prost(bytes = "vec", tag = "4")]
    pub log_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "super::common::ConfigType", tag = "5")]
    pub log_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogId {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub device_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogTime {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub device_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "LogStatus", optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRange {
    #[prost(int64, tag = "1")]
    pub begin: i64,
    #[prost(int64, tag = "2")]
    pub end: i64,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub device_id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "LogStatus", optional, tag = "4")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogUpdate {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(bytes = "vec", tag = "2")]
    pub device_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "LogStatus", optional, tag = "3")]
    pub status: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub log_bytes: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "super::common::ConfigType", optional, tag = "5")]
    pub log_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogReadResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<LogSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogListResponse {
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<LogSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogChangeResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogStatus {
    Default = 0,
    Success = 1,
    ErrorRaw = 2,
    ErrorMissing = 3,
    ErrorConversion = 4,
    ErrorAnalyze = 5,
    ErrorNetwork = 6,
    FailRead = 7,
    FailCreate = 8,
    FailUpdate = 9,
    FailDelete = 10,
    InvalidToken = 11,
    InvalidRequest = 12,
    NotFound = 13,
    MethodNotAllowed = 14,
    UnknownError = 15,
    UnknownStatus = 16,
}
impl LogStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LogStatus::Default => "DEFAULT",
            LogStatus::Success => "SUCCESS",
            LogStatus::ErrorRaw => "ERROR_RAW",
            LogStatus::ErrorMissing => "ERROR_MISSING",
            LogStatus::ErrorConversion => "ERROR_CONVERSION",
            LogStatus::ErrorAnalyze => "ERROR_ANALYZE",
            LogStatus::ErrorNetwork => "ERROR_NETWORK",
            LogStatus::FailRead => "FAIL_READ",
            LogStatus::FailCreate => "FAIL_CREATE",
            LogStatus::FailUpdate => "FAIL_UPDATE",
            LogStatus::FailDelete => "FAIL_DELETE",
            LogStatus::InvalidToken => "INVALID_TOKEN",
            LogStatus::InvalidRequest => "INVALID_REQUEST",
            LogStatus::NotFound => "NOT_FOUND",
            LogStatus::MethodNotAllowed => "METHOD_NOT_ALLOWED",
            LogStatus::UnknownError => "UNKNOWN_ERROR",
            LogStatus::UnknownStatus => "UNKNOWN_STATUS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "SUCCESS" => Some(Self::Success),
            "ERROR_RAW" => Some(Self::ErrorRaw),
            "ERROR_MISSING" => Some(Self::ErrorMissing),
            "ERROR_CONVERSION" => Some(Self::ErrorConversion),
            "ERROR_ANALYZE" => Some(Self::ErrorAnalyze),
            "ERROR_NETWORK" => Some(Self::ErrorNetwork),
            "FAIL_READ" => Some(Self::FailRead),
            "FAIL_CREATE" => Some(Self::FailCreate),
            "FAIL_UPDATE" => Some(Self::FailUpdate),
            "FAIL_DELETE" => Some(Self::FailDelete),
            "INVALID_TOKEN" => Some(Self::InvalidToken),
            "INVALID_REQUEST" => Some(Self::InvalidRequest),
            "NOT_FOUND" => Some(Self::NotFound),
            "METHOD_NOT_ALLOWED" => Some(Self::MethodNotAllowed),
            "UNKNOWN_ERROR" => Some(Self::UnknownError),
            "UNKNOWN_STATUS" => Some(Self::UnknownStatus),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod log_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LogServiceClient<tonic::transport::Channel> {
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
    impl<T> LogServiceClient<T>
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
        ) -> LogServiceClient<InterceptedService<T, F>>
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
            LogServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn read_log(
            &mut self,
            request: impl tonic::IntoRequest<super::LogId>,
        ) -> std::result::Result<
            tonic::Response<super::LogReadResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/log.LogService/ReadLog");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("log.LogService", "ReadLog"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_log_by_time(
            &mut self,
            request: impl tonic::IntoRequest<super::LogTime>,
        ) -> std::result::Result<
            tonic::Response<super::LogListResponse>,
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
                "/log.LogService/ListLogByTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("log.LogService", "ListLogByTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_log_by_last_time(
            &mut self,
            request: impl tonic::IntoRequest<super::LogTime>,
        ) -> std::result::Result<
            tonic::Response<super::LogListResponse>,
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
                "/log.LogService/ListLogByLastTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("log.LogService", "ListLogByLastTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_log_by_range_time(
            &mut self,
            request: impl tonic::IntoRequest<super::LogRange>,
        ) -> std::result::Result<
            tonic::Response<super::LogListResponse>,
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
                "/log.LogService/ListLogByRangeTime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("log.LogService", "ListLogByRangeTime"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_log(
            &mut self,
            request: impl tonic::IntoRequest<super::LogSchema>,
        ) -> std::result::Result<
            tonic::Response<super::LogChangeResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/log.LogService/CreateLog");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("log.LogService", "CreateLog"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_log(
            &mut self,
            request: impl tonic::IntoRequest<super::LogUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::LogChangeResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/log.LogService/UpdateLog");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("log.LogService", "UpdateLog"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_log(
            &mut self,
            request: impl tonic::IntoRequest<super::LogId>,
        ) -> std::result::Result<
            tonic::Response<super::LogChangeResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/log.LogService/DeleteLog");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("log.LogService", "DeleteLog"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod log_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LogServiceServer.
    #[async_trait]
    pub trait LogService: Send + Sync + 'static {
        async fn read_log(
            &self,
            request: tonic::Request<super::LogId>,
        ) -> std::result::Result<tonic::Response<super::LogReadResponse>, tonic::Status>;
        async fn list_log_by_time(
            &self,
            request: tonic::Request<super::LogTime>,
        ) -> std::result::Result<tonic::Response<super::LogListResponse>, tonic::Status>;
        async fn list_log_by_last_time(
            &self,
            request: tonic::Request<super::LogTime>,
        ) -> std::result::Result<tonic::Response<super::LogListResponse>, tonic::Status>;
        async fn list_log_by_range_time(
            &self,
            request: tonic::Request<super::LogRange>,
        ) -> std::result::Result<tonic::Response<super::LogListResponse>, tonic::Status>;
        async fn create_log(
            &self,
            request: tonic::Request<super::LogSchema>,
        ) -> std::result::Result<
            tonic::Response<super::LogChangeResponse>,
            tonic::Status,
        >;
        async fn update_log(
            &self,
            request: tonic::Request<super::LogUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::LogChangeResponse>,
            tonic::Status,
        >;
        async fn delete_log(
            &self,
            request: tonic::Request<super::LogId>,
        ) -> std::result::Result<
            tonic::Response<super::LogChangeResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct LogServiceServer<T: LogService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: LogService> LogServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LogServiceServer<T>
    where
        T: LogService,
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
                "/log.LogService/ReadLog" => {
                    #[allow(non_camel_case_types)]
                    struct ReadLogSvc<T: LogService>(pub Arc<T>);
                    impl<T: LogService> tonic::server::UnaryService<super::LogId>
                    for ReadLogSvc<T> {
                        type Response = super::LogReadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).read_log(request).await };
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
                        let method = ReadLogSvc(inner);
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
                "/log.LogService/ListLogByTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListLogByTimeSvc<T: LogService>(pub Arc<T>);
                    impl<T: LogService> tonic::server::UnaryService<super::LogTime>
                    for ListLogByTimeSvc<T> {
                        type Response = super::LogListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogTime>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_log_by_time(request).await
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
                        let method = ListLogByTimeSvc(inner);
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
                "/log.LogService/ListLogByLastTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListLogByLastTimeSvc<T: LogService>(pub Arc<T>);
                    impl<T: LogService> tonic::server::UnaryService<super::LogTime>
                    for ListLogByLastTimeSvc<T> {
                        type Response = super::LogListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogTime>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_log_by_last_time(request).await
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
                        let method = ListLogByLastTimeSvc(inner);
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
                "/log.LogService/ListLogByRangeTime" => {
                    #[allow(non_camel_case_types)]
                    struct ListLogByRangeTimeSvc<T: LogService>(pub Arc<T>);
                    impl<T: LogService> tonic::server::UnaryService<super::LogRange>
                    for ListLogByRangeTimeSvc<T> {
                        type Response = super::LogListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogRange>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_log_by_range_time(request).await
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
                        let method = ListLogByRangeTimeSvc(inner);
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
                "/log.LogService/CreateLog" => {
                    #[allow(non_camel_case_types)]
                    struct CreateLogSvc<T: LogService>(pub Arc<T>);
                    impl<T: LogService> tonic::server::UnaryService<super::LogSchema>
                    for CreateLogSvc<T> {
                        type Response = super::LogChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogSchema>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_log(request).await };
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
                        let method = CreateLogSvc(inner);
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
                "/log.LogService/UpdateLog" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateLogSvc<T: LogService>(pub Arc<T>);
                    impl<T: LogService> tonic::server::UnaryService<super::LogUpdate>
                    for UpdateLogSvc<T> {
                        type Response = super::LogChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogUpdate>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_log(request).await };
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
                        let method = UpdateLogSvc(inner);
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
                "/log.LogService/DeleteLog" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteLogSvc<T: LogService>(pub Arc<T>);
                    impl<T: LogService> tonic::server::UnaryService<super::LogId>
                    for DeleteLogSvc<T> {
                        type Response = super::LogChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogId>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_log(request).await };
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
                        let method = DeleteLogSvc(inner);
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
    impl<T: LogService> Clone for LogServiceServer<T> {
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
    impl<T: LogService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LogService> tonic::server::NamedService for LogServiceServer<T> {
        const NAME: &'static str = "log.LogService";
    }
}
