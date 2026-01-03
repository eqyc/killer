// @generated
/// Generated client implementations.
pub mod controlling_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ControllingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ControllingServiceClient<tonic::transport::Channel> {
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
    impl<T> ControllingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> ControllingServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            ControllingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_cost_document(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCostDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCostDocumentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/CreateCostDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "CreateCostDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cost_document(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCostDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostDocumentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetCostDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetCostDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_cost_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCostDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCostDocumentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ListCostDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ListCostDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reverse_cost_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ReverseCostDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReverseCostDocumentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ReverseCostDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ReverseCostDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_cost_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateCostDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateCostDocumentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/BatchCreateCostDocuments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "BatchCreateCostDocuments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cost_center(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCostCenterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostCenterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetCostCenter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetCostCenter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_cost_centers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCostCentersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCostCentersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ListCostCenters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ListCostCenters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_cost_center_posting(
            &mut self,
            request: impl tonic::IntoRequest<super::PostCostCenterPostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostCostCenterPostingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/PostCostCenterPosting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "PostCostCenterPosting",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_cost_center_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::PostCostCenterTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostCostCenterTransferResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/PostCostCenterTransfer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "PostCostCenterTransfer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_cost_center_reposting(
            &mut self,
            request: impl tonic::IntoRequest<super::PostCostCenterRepostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostCostCenterRepostingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/PostCostCenterReposting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "PostCostCenterReposting",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cost_center_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCostCenterReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostCenterReportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetCostCenterReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetCostCenterReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cost_center_plan_actual(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCostCenterPlanActualRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostCenterPlanActualResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetCostCenterPlanActual",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetCostCenterPlanActual",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_internal_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInternalOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/CreateInternalOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "CreateInternalOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_internal_order(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInternalOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetInternalOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetInternalOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_internal_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInternalOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInternalOrdersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ListInternalOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ListInternalOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn release_internal_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReleaseInternalOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ReleaseInternalOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ReleaseInternalOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_internal_order_posting(
            &mut self,
            request: impl tonic::IntoRequest<super::PostInternalOrderPostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostInternalOrderPostingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/PostInternalOrderPosting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "PostInternalOrderPosting",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn settle_internal_order(
            &mut self,
            request: impl tonic::IntoRequest<super::SettleInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SettleInternalOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/SettleInternalOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "SettleInternalOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn close_internal_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloseInternalOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/CloseInternalOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "CloseInternalOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_internal_order_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInternalOrderReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInternalOrderReportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetInternalOrderReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetInternalOrderReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_activity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActivityTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetActivityTypeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetActivityType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetActivityType",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_activity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListActivityTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListActivityTypesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ListActivityTypes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ListActivityTypes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_activity_allocation(
            &mut self,
            request: impl tonic::IntoRequest<super::PostActivityAllocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostActivityAllocationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/PostActivityAllocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "PostActivityAllocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_statistical_key_figure(
            &mut self,
            request: impl tonic::IntoRequest<super::PostStatisticalKeyFigureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostStatisticalKeyFigureResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/PostStatisticalKeyFigure",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "PostStatisticalKeyFigure",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn calculate_activity_price(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateActivityPriceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CalculateActivityPriceResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/CalculateActivityPrice",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "CalculateActivityPrice",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_allocation_cycle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllocationCycleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAllocationCycleResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetAllocationCycle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetAllocationCycle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteAssessmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteAssessmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ExecuteAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ExecuteAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_distribution(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteDistributionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteDistributionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ExecuteDistribution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ExecuteDistribution",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_periodic_reposting(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecutePeriodicRepostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutePeriodicRepostingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ExecutePeriodicReposting",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ExecutePeriodicReposting",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_profitability_document(
            &mut self,
            request: impl tonic::IntoRequest<super::PostProfitabilityDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostProfitabilityDocumentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/PostProfitabilityDocument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "PostProfitabilityDocument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_profitability_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProfitabilityReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProfitabilityReportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetProfitabilityReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetProfitabilityReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_profit_center(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProfitCenterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProfitCenterResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetProfitCenter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetProfitCenter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_profit_centers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProfitCentersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProfitCentersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ListProfitCenters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ListProfitCenters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_profit_center_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProfitCenterReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProfitCenterReportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/GetProfitCenterReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "GetProfitCenterReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_period_close(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecutePeriodCloseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutePeriodCloseResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ExecutePeriodClose",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ExecutePeriodClose",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_actual_cost_split(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteActualCostSplitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteActualCostSplitResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ExecuteActualCostSplit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ExecuteActualCostSplit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_wip_calculation(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteWipCalculationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteWipCalculationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ExecuteWIPCalculation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ExecuteWIPCalculation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_variance_calculation(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteVarianceCalculationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteVarianceCalculationResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.controlling.v1.ControllingService/ExecuteVarianceCalculation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.controlling.v1.ControllingService",
                        "ExecuteVarianceCalculation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod controlling_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ControllingServiceServer.
    #[async_trait]
    pub trait ControllingService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_cost_document(
            &self,
            request: tonic::Request<super::CreateCostDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCostDocumentResponse>,
            tonic::Status,
        >;
        async fn get_cost_document(
            &self,
            request: tonic::Request<super::GetCostDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostDocumentResponse>,
            tonic::Status,
        >;
        async fn list_cost_documents(
            &self,
            request: tonic::Request<super::ListCostDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCostDocumentsResponse>,
            tonic::Status,
        >;
        async fn reverse_cost_document(
            &self,
            request: tonic::Request<super::ReverseCostDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReverseCostDocumentResponse>,
            tonic::Status,
        >;
        async fn batch_create_cost_documents(
            &self,
            request: tonic::Request<super::BatchCreateCostDocumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateCostDocumentsResponse>,
            tonic::Status,
        >;
        async fn get_cost_center(
            &self,
            request: tonic::Request<super::GetCostCenterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostCenterResponse>,
            tonic::Status,
        >;
        async fn list_cost_centers(
            &self,
            request: tonic::Request<super::ListCostCentersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCostCentersResponse>,
            tonic::Status,
        >;
        async fn post_cost_center_posting(
            &self,
            request: tonic::Request<super::PostCostCenterPostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostCostCenterPostingResponse>,
            tonic::Status,
        >;
        async fn post_cost_center_transfer(
            &self,
            request: tonic::Request<super::PostCostCenterTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostCostCenterTransferResponse>,
            tonic::Status,
        >;
        async fn post_cost_center_reposting(
            &self,
            request: tonic::Request<super::PostCostCenterRepostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostCostCenterRepostingResponse>,
            tonic::Status,
        >;
        async fn get_cost_center_report(
            &self,
            request: tonic::Request<super::GetCostCenterReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostCenterReportResponse>,
            tonic::Status,
        >;
        async fn get_cost_center_plan_actual(
            &self,
            request: tonic::Request<super::GetCostCenterPlanActualRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCostCenterPlanActualResponse>,
            tonic::Status,
        >;
        async fn create_internal_order(
            &self,
            request: tonic::Request<super::CreateInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInternalOrderResponse>,
            tonic::Status,
        >;
        async fn get_internal_order(
            &self,
            request: tonic::Request<super::GetInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInternalOrderResponse>,
            tonic::Status,
        >;
        async fn list_internal_orders(
            &self,
            request: tonic::Request<super::ListInternalOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInternalOrdersResponse>,
            tonic::Status,
        >;
        async fn release_internal_order(
            &self,
            request: tonic::Request<super::ReleaseInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReleaseInternalOrderResponse>,
            tonic::Status,
        >;
        async fn post_internal_order_posting(
            &self,
            request: tonic::Request<super::PostInternalOrderPostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostInternalOrderPostingResponse>,
            tonic::Status,
        >;
        async fn settle_internal_order(
            &self,
            request: tonic::Request<super::SettleInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SettleInternalOrderResponse>,
            tonic::Status,
        >;
        async fn close_internal_order(
            &self,
            request: tonic::Request<super::CloseInternalOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloseInternalOrderResponse>,
            tonic::Status,
        >;
        async fn get_internal_order_report(
            &self,
            request: tonic::Request<super::GetInternalOrderReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInternalOrderReportResponse>,
            tonic::Status,
        >;
        async fn get_activity_type(
            &self,
            request: tonic::Request<super::GetActivityTypeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetActivityTypeResponse>,
            tonic::Status,
        >;
        async fn list_activity_types(
            &self,
            request: tonic::Request<super::ListActivityTypesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListActivityTypesResponse>,
            tonic::Status,
        >;
        async fn post_activity_allocation(
            &self,
            request: tonic::Request<super::PostActivityAllocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostActivityAllocationResponse>,
            tonic::Status,
        >;
        async fn post_statistical_key_figure(
            &self,
            request: tonic::Request<super::PostStatisticalKeyFigureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostStatisticalKeyFigureResponse>,
            tonic::Status,
        >;
        async fn calculate_activity_price(
            &self,
            request: tonic::Request<super::CalculateActivityPriceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CalculateActivityPriceResponse>,
            tonic::Status,
        >;
        async fn get_allocation_cycle(
            &self,
            request: tonic::Request<super::GetAllocationCycleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAllocationCycleResponse>,
            tonic::Status,
        >;
        async fn execute_assessment(
            &self,
            request: tonic::Request<super::ExecuteAssessmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteAssessmentResponse>,
            tonic::Status,
        >;
        async fn execute_distribution(
            &self,
            request: tonic::Request<super::ExecuteDistributionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteDistributionResponse>,
            tonic::Status,
        >;
        async fn execute_periodic_reposting(
            &self,
            request: tonic::Request<super::ExecutePeriodicRepostingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutePeriodicRepostingResponse>,
            tonic::Status,
        >;
        async fn post_profitability_document(
            &self,
            request: tonic::Request<super::PostProfitabilityDocumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostProfitabilityDocumentResponse>,
            tonic::Status,
        >;
        async fn get_profitability_report(
            &self,
            request: tonic::Request<super::GetProfitabilityReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProfitabilityReportResponse>,
            tonic::Status,
        >;
        async fn get_profit_center(
            &self,
            request: tonic::Request<super::GetProfitCenterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProfitCenterResponse>,
            tonic::Status,
        >;
        async fn list_profit_centers(
            &self,
            request: tonic::Request<super::ListProfitCentersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProfitCentersResponse>,
            tonic::Status,
        >;
        async fn get_profit_center_report(
            &self,
            request: tonic::Request<super::GetProfitCenterReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProfitCenterReportResponse>,
            tonic::Status,
        >;
        async fn execute_period_close(
            &self,
            request: tonic::Request<super::ExecutePeriodCloseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecutePeriodCloseResponse>,
            tonic::Status,
        >;
        async fn execute_actual_cost_split(
            &self,
            request: tonic::Request<super::ExecuteActualCostSplitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteActualCostSplitResponse>,
            tonic::Status,
        >;
        async fn execute_wip_calculation(
            &self,
            request: tonic::Request<super::ExecuteWipCalculationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteWipCalculationResponse>,
            tonic::Status,
        >;
        async fn execute_variance_calculation(
            &self,
            request: tonic::Request<super::ExecuteVarianceCalculationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteVarianceCalculationResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ControllingServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> ControllingServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ControllingServiceServer<T>
    where
        T: ControllingService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/finance.controlling.v1.ControllingService/CreateCostDocument" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCostDocumentSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::CreateCostDocumentRequest>
                    for CreateCostDocumentSvc<T> {
                        type Response = super::CreateCostDocumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCostDocumentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::create_cost_document(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateCostDocumentSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetCostDocument" => {
                    #[allow(non_camel_case_types)]
                    struct GetCostDocumentSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetCostDocumentRequest>
                    for GetCostDocumentSvc<T> {
                        type Response = super::GetCostDocumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCostDocumentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_cost_document(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetCostDocumentSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ListCostDocuments" => {
                    #[allow(non_camel_case_types)]
                    struct ListCostDocumentsSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ListCostDocumentsRequest>
                    for ListCostDocumentsSvc<T> {
                        type Response = super::ListCostDocumentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCostDocumentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::list_cost_documents(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListCostDocumentsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ReverseCostDocument" => {
                    #[allow(non_camel_case_types)]
                    struct ReverseCostDocumentSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ReverseCostDocumentRequest>
                    for ReverseCostDocumentSvc<T> {
                        type Response = super::ReverseCostDocumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReverseCostDocumentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::reverse_cost_document(
                                        &inner,
                                        request,
                                    )
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
                        let method = ReverseCostDocumentSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/BatchCreateCostDocuments" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateCostDocumentsSvc<T: ControllingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::BatchCreateCostDocumentsRequest>
                    for BatchCreateCostDocumentsSvc<T> {
                        type Response = super::BatchCreateCostDocumentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchCreateCostDocumentsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::batch_create_cost_documents(
                                        &inner,
                                        request,
                                    )
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
                        let method = BatchCreateCostDocumentsSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetCostCenter" => {
                    #[allow(non_camel_case_types)]
                    struct GetCostCenterSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetCostCenterRequest>
                    for GetCostCenterSvc<T> {
                        type Response = super::GetCostCenterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCostCenterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_cost_center(&inner, request)
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
                        let method = GetCostCenterSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ListCostCenters" => {
                    #[allow(non_camel_case_types)]
                    struct ListCostCentersSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ListCostCentersRequest>
                    for ListCostCentersSvc<T> {
                        type Response = super::ListCostCentersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCostCentersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::list_cost_centers(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListCostCentersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/PostCostCenterPosting" => {
                    #[allow(non_camel_case_types)]
                    struct PostCostCenterPostingSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::PostCostCenterPostingRequest>
                    for PostCostCenterPostingSvc<T> {
                        type Response = super::PostCostCenterPostingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostCostCenterPostingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::post_cost_center_posting(
                                        &inner,
                                        request,
                                    )
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
                        let method = PostCostCenterPostingSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/PostCostCenterTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct PostCostCenterTransferSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::PostCostCenterTransferRequest>
                    for PostCostCenterTransferSvc<T> {
                        type Response = super::PostCostCenterTransferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostCostCenterTransferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::post_cost_center_transfer(
                                        &inner,
                                        request,
                                    )
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
                        let method = PostCostCenterTransferSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/PostCostCenterReposting" => {
                    #[allow(non_camel_case_types)]
                    struct PostCostCenterRepostingSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::PostCostCenterRepostingRequest>
                    for PostCostCenterRepostingSvc<T> {
                        type Response = super::PostCostCenterRepostingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PostCostCenterRepostingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::post_cost_center_reposting(
                                        &inner,
                                        request,
                                    )
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
                        let method = PostCostCenterRepostingSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetCostCenterReport" => {
                    #[allow(non_camel_case_types)]
                    struct GetCostCenterReportSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetCostCenterReportRequest>
                    for GetCostCenterReportSvc<T> {
                        type Response = super::GetCostCenterReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCostCenterReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_cost_center_report(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetCostCenterReportSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetCostCenterPlanActual" => {
                    #[allow(non_camel_case_types)]
                    struct GetCostCenterPlanActualSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetCostCenterPlanActualRequest>
                    for GetCostCenterPlanActualSvc<T> {
                        type Response = super::GetCostCenterPlanActualResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetCostCenterPlanActualRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_cost_center_plan_actual(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetCostCenterPlanActualSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/CreateInternalOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInternalOrderSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::CreateInternalOrderRequest>
                    for CreateInternalOrderSvc<T> {
                        type Response = super::CreateInternalOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInternalOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::create_internal_order(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateInternalOrderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetInternalOrder" => {
                    #[allow(non_camel_case_types)]
                    struct GetInternalOrderSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetInternalOrderRequest>
                    for GetInternalOrderSvc<T> {
                        type Response = super::GetInternalOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInternalOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_internal_order(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetInternalOrderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ListInternalOrders" => {
                    #[allow(non_camel_case_types)]
                    struct ListInternalOrdersSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ListInternalOrdersRequest>
                    for ListInternalOrdersSvc<T> {
                        type Response = super::ListInternalOrdersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInternalOrdersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::list_internal_orders(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListInternalOrdersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ReleaseInternalOrder" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseInternalOrderSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ReleaseInternalOrderRequest>
                    for ReleaseInternalOrderSvc<T> {
                        type Response = super::ReleaseInternalOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReleaseInternalOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::release_internal_order(
                                        &inner,
                                        request,
                                    )
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
                        let method = ReleaseInternalOrderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/PostInternalOrderPosting" => {
                    #[allow(non_camel_case_types)]
                    struct PostInternalOrderPostingSvc<T: ControllingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::PostInternalOrderPostingRequest>
                    for PostInternalOrderPostingSvc<T> {
                        type Response = super::PostInternalOrderPostingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PostInternalOrderPostingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::post_internal_order_posting(
                                        &inner,
                                        request,
                                    )
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
                        let method = PostInternalOrderPostingSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/SettleInternalOrder" => {
                    #[allow(non_camel_case_types)]
                    struct SettleInternalOrderSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::SettleInternalOrderRequest>
                    for SettleInternalOrderSvc<T> {
                        type Response = super::SettleInternalOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SettleInternalOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::settle_internal_order(
                                        &inner,
                                        request,
                                    )
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
                        let method = SettleInternalOrderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/CloseInternalOrder" => {
                    #[allow(non_camel_case_types)]
                    struct CloseInternalOrderSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::CloseInternalOrderRequest>
                    for CloseInternalOrderSvc<T> {
                        type Response = super::CloseInternalOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloseInternalOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::close_internal_order(
                                        &inner,
                                        request,
                                    )
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
                        let method = CloseInternalOrderSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetInternalOrderReport" => {
                    #[allow(non_camel_case_types)]
                    struct GetInternalOrderReportSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetInternalOrderReportRequest>
                    for GetInternalOrderReportSvc<T> {
                        type Response = super::GetInternalOrderReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInternalOrderReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_internal_order_report(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetInternalOrderReportSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetActivityType" => {
                    #[allow(non_camel_case_types)]
                    struct GetActivityTypeSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetActivityTypeRequest>
                    for GetActivityTypeSvc<T> {
                        type Response = super::GetActivityTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetActivityTypeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_activity_type(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetActivityTypeSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ListActivityTypes" => {
                    #[allow(non_camel_case_types)]
                    struct ListActivityTypesSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ListActivityTypesRequest>
                    for ListActivityTypesSvc<T> {
                        type Response = super::ListActivityTypesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListActivityTypesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::list_activity_types(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListActivityTypesSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/PostActivityAllocation" => {
                    #[allow(non_camel_case_types)]
                    struct PostActivityAllocationSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::PostActivityAllocationRequest>
                    for PostActivityAllocationSvc<T> {
                        type Response = super::PostActivityAllocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostActivityAllocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::post_activity_allocation(
                                        &inner,
                                        request,
                                    )
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
                        let method = PostActivityAllocationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/PostStatisticalKeyFigure" => {
                    #[allow(non_camel_case_types)]
                    struct PostStatisticalKeyFigureSvc<T: ControllingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::PostStatisticalKeyFigureRequest>
                    for PostStatisticalKeyFigureSvc<T> {
                        type Response = super::PostStatisticalKeyFigureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PostStatisticalKeyFigureRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::post_statistical_key_figure(
                                        &inner,
                                        request,
                                    )
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
                        let method = PostStatisticalKeyFigureSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/CalculateActivityPrice" => {
                    #[allow(non_camel_case_types)]
                    struct CalculateActivityPriceSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::CalculateActivityPriceRequest>
                    for CalculateActivityPriceSvc<T> {
                        type Response = super::CalculateActivityPriceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CalculateActivityPriceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::calculate_activity_price(
                                        &inner,
                                        request,
                                    )
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
                        let method = CalculateActivityPriceSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetAllocationCycle" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllocationCycleSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetAllocationCycleRequest>
                    for GetAllocationCycleSvc<T> {
                        type Response = super::GetAllocationCycleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllocationCycleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_allocation_cycle(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetAllocationCycleSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ExecuteAssessment" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteAssessmentSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ExecuteAssessmentRequest>
                    for ExecuteAssessmentSvc<T> {
                        type Response = super::ExecuteAssessmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteAssessmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::execute_assessment(
                                        &inner,
                                        request,
                                    )
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
                        let method = ExecuteAssessmentSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ExecuteDistribution" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteDistributionSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ExecuteDistributionRequest>
                    for ExecuteDistributionSvc<T> {
                        type Response = super::ExecuteDistributionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteDistributionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::execute_distribution(
                                        &inner,
                                        request,
                                    )
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
                        let method = ExecuteDistributionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ExecutePeriodicReposting" => {
                    #[allow(non_camel_case_types)]
                    struct ExecutePeriodicRepostingSvc<T: ControllingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ExecutePeriodicRepostingRequest>
                    for ExecutePeriodicRepostingSvc<T> {
                        type Response = super::ExecutePeriodicRepostingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ExecutePeriodicRepostingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::execute_periodic_reposting(
                                        &inner,
                                        request,
                                    )
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
                        let method = ExecutePeriodicRepostingSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/PostProfitabilityDocument" => {
                    #[allow(non_camel_case_types)]
                    struct PostProfitabilityDocumentSvc<T: ControllingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<
                        super::PostProfitabilityDocumentRequest,
                    > for PostProfitabilityDocumentSvc<T> {
                        type Response = super::PostProfitabilityDocumentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PostProfitabilityDocumentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::post_profitability_document(
                                        &inner,
                                        request,
                                    )
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
                        let method = PostProfitabilityDocumentSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetProfitabilityReport" => {
                    #[allow(non_camel_case_types)]
                    struct GetProfitabilityReportSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetProfitabilityReportRequest>
                    for GetProfitabilityReportSvc<T> {
                        type Response = super::GetProfitabilityReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProfitabilityReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_profitability_report(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetProfitabilityReportSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetProfitCenter" => {
                    #[allow(non_camel_case_types)]
                    struct GetProfitCenterSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetProfitCenterRequest>
                    for GetProfitCenterSvc<T> {
                        type Response = super::GetProfitCenterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProfitCenterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_profit_center(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetProfitCenterSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ListProfitCenters" => {
                    #[allow(non_camel_case_types)]
                    struct ListProfitCentersSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ListProfitCentersRequest>
                    for ListProfitCentersSvc<T> {
                        type Response = super::ListProfitCentersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProfitCentersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::list_profit_centers(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListProfitCentersSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/GetProfitCenterReport" => {
                    #[allow(non_camel_case_types)]
                    struct GetProfitCenterReportSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::GetProfitCenterReportRequest>
                    for GetProfitCenterReportSvc<T> {
                        type Response = super::GetProfitCenterReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProfitCenterReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::get_profit_center_report(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetProfitCenterReportSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ExecutePeriodClose" => {
                    #[allow(non_camel_case_types)]
                    struct ExecutePeriodCloseSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ExecutePeriodCloseRequest>
                    for ExecutePeriodCloseSvc<T> {
                        type Response = super::ExecutePeriodCloseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecutePeriodCloseRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::execute_period_close(
                                        &inner,
                                        request,
                                    )
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
                        let method = ExecutePeriodCloseSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ExecuteActualCostSplit" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteActualCostSplitSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ExecuteActualCostSplitRequest>
                    for ExecuteActualCostSplitSvc<T> {
                        type Response = super::ExecuteActualCostSplitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteActualCostSplitRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::execute_actual_cost_split(
                                        &inner,
                                        request,
                                    )
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
                        let method = ExecuteActualCostSplitSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ExecuteWIPCalculation" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteWIPCalculationSvc<T: ControllingService>(pub Arc<T>);
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<super::ExecuteWipCalculationRequest>
                    for ExecuteWIPCalculationSvc<T> {
                        type Response = super::ExecuteWipCalculationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteWipCalculationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::execute_wip_calculation(
                                        &inner,
                                        request,
                                    )
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
                        let method = ExecuteWIPCalculationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/finance.controlling.v1.ControllingService/ExecuteVarianceCalculation" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteVarianceCalculationSvc<T: ControllingService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ControllingService,
                    > tonic::server::UnaryService<
                        super::ExecuteVarianceCalculationRequest,
                    > for ExecuteVarianceCalculationSvc<T> {
                        type Response = super::ExecuteVarianceCalculationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ExecuteVarianceCalculationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ControllingService>::execute_variance_calculation(
                                        &inner,
                                        request,
                                    )
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
                        let method = ExecuteVarianceCalculationSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                        let mut response = http::Response::new(
                            tonic::body::Body::default(),
                        );
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for ControllingServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "finance.controlling.v1.ControllingService";
    impl<T> tonic::server::NamedService for ControllingServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
