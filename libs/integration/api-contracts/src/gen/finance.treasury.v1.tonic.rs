// @generated
/// Generated client implementations.
pub mod treasury_service_client {
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
    pub struct TreasuryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TreasuryServiceClient<tonic::transport::Channel> {
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
    impl<T> TreasuryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
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
        ) -> TreasuryServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            TreasuryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_bank_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBankAccountResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetBankAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetBankAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_bank_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBankAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBankAccountsResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ListBankAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ListBankAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_bank_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBankAccountResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CreateBankAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "CreateBankAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_bank_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateBankAccountResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/UpdateBankAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "UpdateBankAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountBalanceResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetAccountBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetAccountBalance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_account_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccountTransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccountTransactionsResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ListAccountTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ListAccountTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_cash_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCashFlowResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CreateCashFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "CreateCashFlow",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cash_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCashFlowResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetCashFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.treasury.v1.TreasuryService", "GetCashFlow"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_cash_flows(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCashFlowsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCashFlowsResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ListCashFlows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ListCashFlows",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn confirm_cash_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmCashFlowResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ConfirmCashFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ConfirmCashFlow",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_cash_flow(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelCashFlowResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CancelCashFlow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "CancelCashFlow",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_liquidity_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLiquidityReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLiquidityReportResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetLiquidityReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetLiquidityReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_liquidity_forecast(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateLiquidityForecastRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateLiquidityForecastResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GenerateLiquidityForecast",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GenerateLiquidityForecast",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cash_flow_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCashFlowSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCashFlowSummaryResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetCashFlowSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetCashFlowSummary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_cash_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCashPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCashPoolResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CreateCashPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "CreateCashPool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_cash_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteCashPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteCashPoolResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ExecuteCashPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ExecuteCashPool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_cash_pool_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCashPoolStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCashPoolStatusResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetCashPoolStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetCashPoolStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_treasury_position(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTreasuryPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTreasuryPositionResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetTreasuryPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetTreasuryPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_position_history(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPositionHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPositionHistoryResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ListPositionHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ListPositionHistory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn adjust_position(
            &mut self,
            request: impl tonic::IntoRequest<super::AdjustPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AdjustPositionResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/AdjustPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "AdjustPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_term_deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTermDepositRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTermDepositResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetTermDeposit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetTermDeposit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_term_deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTermDepositRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTermDepositResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CreateTermDeposit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "CreateTermDeposit",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn process_maturity(
            &mut self,
            request: impl tonic::IntoRequest<super::ProcessMaturityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProcessMaturityResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ProcessMaturity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ProcessMaturity",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_investments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInvestmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInvestmentsResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ListInvestments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ListInvestments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_loan(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLoanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLoanResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetLoan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.treasury.v1.TreasuryService", "GetLoan"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_loan(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLoanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateLoanResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CreateLoan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.treasury.v1.TreasuryService", "CreateLoan"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn drawdown_loan(
            &mut self,
            request: impl tonic::IntoRequest<super::DrawdownLoanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DrawdownLoanResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/DrawdownLoan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "DrawdownLoan",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn repay_loan(
            &mut self,
            request: impl tonic::IntoRequest<super::RepayLoanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RepayLoanResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/RepayLoan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.treasury.v1.TreasuryService", "RepayLoan"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_loans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLoansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLoansResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ListLoans",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.treasury.v1.TreasuryService", "ListLoans"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_fx_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFxTransactionResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetFXTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetFXTransaction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_fx_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFxTransactionResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CreateFXTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "CreateFXTransaction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_fx_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteFxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteFxTransactionResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/ExecuteFXTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "ExecuteFXTransaction",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_fx_exposure(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFxExposureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFxExposureResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetFXExposure",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetFXExposure",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn revalue_fx_position(
            &mut self,
            request: impl tonic::IntoRequest<super::RevalueFxPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevalueFxPositionResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/RevalueFXPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "RevalueFXPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_internal_bank_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInternalBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInternalBankAccountResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/CreateInternalBankAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "CreateInternalBankAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn internal_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::InternalTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InternalTransferResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/InternalTransfer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "InternalTransfer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_internal_bank_statement(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInternalBankStatementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInternalBankStatementResponse>,
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
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.treasury.v1.TreasuryService/GetInternalBankStatement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.treasury.v1.TreasuryService",
                        "GetInternalBankStatement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod treasury_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TreasuryServiceServer.
    #[async_trait]
    pub trait TreasuryService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_bank_account(
            &self,
            request: tonic::Request<super::GetBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBankAccountResponse>,
            tonic::Status,
        >;
        async fn list_bank_accounts(
            &self,
            request: tonic::Request<super::ListBankAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBankAccountsResponse>,
            tonic::Status,
        >;
        async fn create_bank_account(
            &self,
            request: tonic::Request<super::CreateBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBankAccountResponse>,
            tonic::Status,
        >;
        async fn update_bank_account(
            &self,
            request: tonic::Request<super::UpdateBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateBankAccountResponse>,
            tonic::Status,
        >;
        async fn get_account_balance(
            &self,
            request: tonic::Request<super::GetAccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountBalanceResponse>,
            tonic::Status,
        >;
        async fn list_account_transactions(
            &self,
            request: tonic::Request<super::ListAccountTransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccountTransactionsResponse>,
            tonic::Status,
        >;
        async fn create_cash_flow(
            &self,
            request: tonic::Request<super::CreateCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCashFlowResponse>,
            tonic::Status,
        >;
        async fn get_cash_flow(
            &self,
            request: tonic::Request<super::GetCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCashFlowResponse>,
            tonic::Status,
        >;
        async fn list_cash_flows(
            &self,
            request: tonic::Request<super::ListCashFlowsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCashFlowsResponse>,
            tonic::Status,
        >;
        async fn confirm_cash_flow(
            &self,
            request: tonic::Request<super::ConfirmCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfirmCashFlowResponse>,
            tonic::Status,
        >;
        async fn cancel_cash_flow(
            &self,
            request: tonic::Request<super::CancelCashFlowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelCashFlowResponse>,
            tonic::Status,
        >;
        async fn get_liquidity_report(
            &self,
            request: tonic::Request<super::GetLiquidityReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLiquidityReportResponse>,
            tonic::Status,
        >;
        async fn generate_liquidity_forecast(
            &self,
            request: tonic::Request<super::GenerateLiquidityForecastRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateLiquidityForecastResponse>,
            tonic::Status,
        >;
        async fn get_cash_flow_summary(
            &self,
            request: tonic::Request<super::GetCashFlowSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCashFlowSummaryResponse>,
            tonic::Status,
        >;
        async fn create_cash_pool(
            &self,
            request: tonic::Request<super::CreateCashPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCashPoolResponse>,
            tonic::Status,
        >;
        async fn execute_cash_pool(
            &self,
            request: tonic::Request<super::ExecuteCashPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteCashPoolResponse>,
            tonic::Status,
        >;
        async fn get_cash_pool_status(
            &self,
            request: tonic::Request<super::GetCashPoolStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCashPoolStatusResponse>,
            tonic::Status,
        >;
        async fn get_treasury_position(
            &self,
            request: tonic::Request<super::GetTreasuryPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTreasuryPositionResponse>,
            tonic::Status,
        >;
        async fn list_position_history(
            &self,
            request: tonic::Request<super::ListPositionHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPositionHistoryResponse>,
            tonic::Status,
        >;
        async fn adjust_position(
            &self,
            request: tonic::Request<super::AdjustPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AdjustPositionResponse>,
            tonic::Status,
        >;
        async fn get_term_deposit(
            &self,
            request: tonic::Request<super::GetTermDepositRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTermDepositResponse>,
            tonic::Status,
        >;
        async fn create_term_deposit(
            &self,
            request: tonic::Request<super::CreateTermDepositRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateTermDepositResponse>,
            tonic::Status,
        >;
        async fn process_maturity(
            &self,
            request: tonic::Request<super::ProcessMaturityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProcessMaturityResponse>,
            tonic::Status,
        >;
        async fn list_investments(
            &self,
            request: tonic::Request<super::ListInvestmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInvestmentsResponse>,
            tonic::Status,
        >;
        async fn get_loan(
            &self,
            request: tonic::Request<super::GetLoanRequest>,
        ) -> std::result::Result<tonic::Response<super::GetLoanResponse>, tonic::Status>;
        async fn create_loan(
            &self,
            request: tonic::Request<super::CreateLoanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateLoanResponse>,
            tonic::Status,
        >;
        async fn drawdown_loan(
            &self,
            request: tonic::Request<super::DrawdownLoanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DrawdownLoanResponse>,
            tonic::Status,
        >;
        async fn repay_loan(
            &self,
            request: tonic::Request<super::RepayLoanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RepayLoanResponse>,
            tonic::Status,
        >;
        async fn list_loans(
            &self,
            request: tonic::Request<super::ListLoansRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLoansResponse>,
            tonic::Status,
        >;
        async fn get_fx_transaction(
            &self,
            request: tonic::Request<super::GetFxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFxTransactionResponse>,
            tonic::Status,
        >;
        async fn create_fx_transaction(
            &self,
            request: tonic::Request<super::CreateFxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFxTransactionResponse>,
            tonic::Status,
        >;
        async fn execute_fx_transaction(
            &self,
            request: tonic::Request<super::ExecuteFxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteFxTransactionResponse>,
            tonic::Status,
        >;
        async fn get_fx_exposure(
            &self,
            request: tonic::Request<super::GetFxExposureRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFxExposureResponse>,
            tonic::Status,
        >;
        async fn revalue_fx_position(
            &self,
            request: tonic::Request<super::RevalueFxPositionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevalueFxPositionResponse>,
            tonic::Status,
        >;
        async fn create_internal_bank_account(
            &self,
            request: tonic::Request<super::CreateInternalBankAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateInternalBankAccountResponse>,
            tonic::Status,
        >;
        async fn internal_transfer(
            &self,
            request: tonic::Request<super::InternalTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InternalTransferResponse>,
            tonic::Status,
        >;
        async fn get_internal_bank_statement(
            &self,
            request: tonic::Request<super::GetInternalBankStatementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInternalBankStatementResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct TreasuryServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> TreasuryServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TreasuryServiceServer<T>
    where
        T: TreasuryService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
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
            match req.uri().path() {
                "/finance.treasury.v1.TreasuryService/GetBankAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetBankAccountSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetBankAccountRequest>
                    for GetBankAccountSvc<T> {
                        type Response = super::GetBankAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBankAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_bank_account(&inner, request)
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
                        let method = GetBankAccountSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ListBankAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct ListBankAccountsSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ListBankAccountsRequest>
                    for ListBankAccountsSvc<T> {
                        type Response = super::ListBankAccountsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBankAccountsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::list_bank_accounts(&inner, request)
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
                        let method = ListBankAccountsSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CreateBankAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBankAccountSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::CreateBankAccountRequest>
                    for CreateBankAccountSvc<T> {
                        type Response = super::CreateBankAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBankAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::create_bank_account(&inner, request)
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
                        let method = CreateBankAccountSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/UpdateBankAccount" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBankAccountSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::UpdateBankAccountRequest>
                    for UpdateBankAccountSvc<T> {
                        type Response = super::UpdateBankAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBankAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::update_bank_account(&inner, request)
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
                        let method = UpdateBankAccountSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetAccountBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountBalanceSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetAccountBalanceRequest>
                    for GetAccountBalanceSvc<T> {
                        type Response = super::GetAccountBalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_account_balance(&inner, request)
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
                        let method = GetAccountBalanceSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ListAccountTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct ListAccountTransactionsSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ListAccountTransactionsRequest>
                    for ListAccountTransactionsSvc<T> {
                        type Response = super::ListAccountTransactionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListAccountTransactionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::list_account_transactions(
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
                        let method = ListAccountTransactionsSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CreateCashFlow" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCashFlowSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::CreateCashFlowRequest>
                    for CreateCashFlowSvc<T> {
                        type Response = super::CreateCashFlowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCashFlowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::create_cash_flow(&inner, request)
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
                        let method = CreateCashFlowSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetCashFlow" => {
                    #[allow(non_camel_case_types)]
                    struct GetCashFlowSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetCashFlowRequest>
                    for GetCashFlowSvc<T> {
                        type Response = super::GetCashFlowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCashFlowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_cash_flow(&inner, request).await
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
                        let method = GetCashFlowSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ListCashFlows" => {
                    #[allow(non_camel_case_types)]
                    struct ListCashFlowsSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ListCashFlowsRequest>
                    for ListCashFlowsSvc<T> {
                        type Response = super::ListCashFlowsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCashFlowsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::list_cash_flows(&inner, request)
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
                        let method = ListCashFlowsSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ConfirmCashFlow" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmCashFlowSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ConfirmCashFlowRequest>
                    for ConfirmCashFlowSvc<T> {
                        type Response = super::ConfirmCashFlowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmCashFlowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::confirm_cash_flow(&inner, request)
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
                        let method = ConfirmCashFlowSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CancelCashFlow" => {
                    #[allow(non_camel_case_types)]
                    struct CancelCashFlowSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::CancelCashFlowRequest>
                    for CancelCashFlowSvc<T> {
                        type Response = super::CancelCashFlowResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelCashFlowRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::cancel_cash_flow(&inner, request)
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
                        let method = CancelCashFlowSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetLiquidityReport" => {
                    #[allow(non_camel_case_types)]
                    struct GetLiquidityReportSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetLiquidityReportRequest>
                    for GetLiquidityReportSvc<T> {
                        type Response = super::GetLiquidityReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLiquidityReportRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_liquidity_report(
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
                        let method = GetLiquidityReportSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GenerateLiquidityForecast" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateLiquidityForecastSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<
                        super::GenerateLiquidityForecastRequest,
                    > for GenerateLiquidityForecastSvc<T> {
                        type Response = super::GenerateLiquidityForecastResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GenerateLiquidityForecastRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::generate_liquidity_forecast(
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
                        let method = GenerateLiquidityForecastSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetCashFlowSummary" => {
                    #[allow(non_camel_case_types)]
                    struct GetCashFlowSummarySvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetCashFlowSummaryRequest>
                    for GetCashFlowSummarySvc<T> {
                        type Response = super::GetCashFlowSummaryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCashFlowSummaryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_cash_flow_summary(
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
                        let method = GetCashFlowSummarySvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CreateCashPool" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCashPoolSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::CreateCashPoolRequest>
                    for CreateCashPoolSvc<T> {
                        type Response = super::CreateCashPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCashPoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::create_cash_pool(&inner, request)
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
                        let method = CreateCashPoolSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ExecuteCashPool" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteCashPoolSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ExecuteCashPoolRequest>
                    for ExecuteCashPoolSvc<T> {
                        type Response = super::ExecuteCashPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteCashPoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::execute_cash_pool(&inner, request)
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
                        let method = ExecuteCashPoolSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetCashPoolStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetCashPoolStatusSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetCashPoolStatusRequest>
                    for GetCashPoolStatusSvc<T> {
                        type Response = super::GetCashPoolStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCashPoolStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_cash_pool_status(
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
                        let method = GetCashPoolStatusSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetTreasuryPosition" => {
                    #[allow(non_camel_case_types)]
                    struct GetTreasuryPositionSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetTreasuryPositionRequest>
                    for GetTreasuryPositionSvc<T> {
                        type Response = super::GetTreasuryPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTreasuryPositionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_treasury_position(
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
                        let method = GetTreasuryPositionSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ListPositionHistory" => {
                    #[allow(non_camel_case_types)]
                    struct ListPositionHistorySvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ListPositionHistoryRequest>
                    for ListPositionHistorySvc<T> {
                        type Response = super::ListPositionHistoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPositionHistoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::list_position_history(
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
                        let method = ListPositionHistorySvc(inner);
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
                "/finance.treasury.v1.TreasuryService/AdjustPosition" => {
                    #[allow(non_camel_case_types)]
                    struct AdjustPositionSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::AdjustPositionRequest>
                    for AdjustPositionSvc<T> {
                        type Response = super::AdjustPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AdjustPositionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::adjust_position(&inner, request)
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
                        let method = AdjustPositionSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetTermDeposit" => {
                    #[allow(non_camel_case_types)]
                    struct GetTermDepositSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetTermDepositRequest>
                    for GetTermDepositSvc<T> {
                        type Response = super::GetTermDepositResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTermDepositRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_term_deposit(&inner, request)
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
                        let method = GetTermDepositSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CreateTermDeposit" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTermDepositSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::CreateTermDepositRequest>
                    for CreateTermDepositSvc<T> {
                        type Response = super::CreateTermDepositResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTermDepositRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::create_term_deposit(&inner, request)
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
                        let method = CreateTermDepositSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ProcessMaturity" => {
                    #[allow(non_camel_case_types)]
                    struct ProcessMaturitySvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ProcessMaturityRequest>
                    for ProcessMaturitySvc<T> {
                        type Response = super::ProcessMaturityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProcessMaturityRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::process_maturity(&inner, request)
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
                        let method = ProcessMaturitySvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ListInvestments" => {
                    #[allow(non_camel_case_types)]
                    struct ListInvestmentsSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ListInvestmentsRequest>
                    for ListInvestmentsSvc<T> {
                        type Response = super::ListInvestmentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInvestmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::list_investments(&inner, request)
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
                        let method = ListInvestmentsSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetLoan" => {
                    #[allow(non_camel_case_types)]
                    struct GetLoanSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetLoanRequest>
                    for GetLoanSvc<T> {
                        type Response = super::GetLoanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLoanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_loan(&inner, request).await
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
                        let method = GetLoanSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CreateLoan" => {
                    #[allow(non_camel_case_types)]
                    struct CreateLoanSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::CreateLoanRequest>
                    for CreateLoanSvc<T> {
                        type Response = super::CreateLoanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateLoanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::create_loan(&inner, request).await
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
                        let method = CreateLoanSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/DrawdownLoan" => {
                    #[allow(non_camel_case_types)]
                    struct DrawdownLoanSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::DrawdownLoanRequest>
                    for DrawdownLoanSvc<T> {
                        type Response = super::DrawdownLoanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DrawdownLoanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::drawdown_loan(&inner, request).await
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
                        let method = DrawdownLoanSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/RepayLoan" => {
                    #[allow(non_camel_case_types)]
                    struct RepayLoanSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::RepayLoanRequest>
                    for RepayLoanSvc<T> {
                        type Response = super::RepayLoanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepayLoanRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::repay_loan(&inner, request).await
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
                        let method = RepayLoanSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ListLoans" => {
                    #[allow(non_camel_case_types)]
                    struct ListLoansSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ListLoansRequest>
                    for ListLoansSvc<T> {
                        type Response = super::ListLoansResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListLoansRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::list_loans(&inner, request).await
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
                        let method = ListLoansSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetFXTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetFXTransactionSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetFxTransactionRequest>
                    for GetFXTransactionSvc<T> {
                        type Response = super::GetFxTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_fx_transaction(&inner, request)
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
                        let method = GetFXTransactionSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CreateFXTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFXTransactionSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::CreateFxTransactionRequest>
                    for CreateFXTransactionSvc<T> {
                        type Response = super::CreateFxTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::create_fx_transaction(
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
                        let method = CreateFXTransactionSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/ExecuteFXTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteFXTransactionSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::ExecuteFxTransactionRequest>
                    for ExecuteFXTransactionSvc<T> {
                        type Response = super::ExecuteFxTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteFxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::execute_fx_transaction(
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
                        let method = ExecuteFXTransactionSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetFXExposure" => {
                    #[allow(non_camel_case_types)]
                    struct GetFXExposureSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetFxExposureRequest>
                    for GetFXExposureSvc<T> {
                        type Response = super::GetFxExposureResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFxExposureRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_fx_exposure(&inner, request)
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
                        let method = GetFXExposureSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/RevalueFXPosition" => {
                    #[allow(non_camel_case_types)]
                    struct RevalueFXPositionSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::RevalueFxPositionRequest>
                    for RevalueFXPositionSvc<T> {
                        type Response = super::RevalueFxPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevalueFxPositionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::revalue_fx_position(&inner, request)
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
                        let method = RevalueFXPositionSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/CreateInternalBankAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInternalBankAccountSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<
                        super::CreateInternalBankAccountRequest,
                    > for CreateInternalBankAccountSvc<T> {
                        type Response = super::CreateInternalBankAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateInternalBankAccountRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::create_internal_bank_account(
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
                        let method = CreateInternalBankAccountSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/InternalTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct InternalTransferSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::InternalTransferRequest>
                    for InternalTransferSvc<T> {
                        type Response = super::InternalTransferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InternalTransferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::internal_transfer(&inner, request)
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
                        let method = InternalTransferSvc(inner);
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
                "/finance.treasury.v1.TreasuryService/GetInternalBankStatement" => {
                    #[allow(non_camel_case_types)]
                    struct GetInternalBankStatementSvc<T: TreasuryService>(pub Arc<T>);
                    impl<
                        T: TreasuryService,
                    > tonic::server::UnaryService<super::GetInternalBankStatementRequest>
                    for GetInternalBankStatementSvc<T> {
                        type Response = super::GetInternalBankStatementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetInternalBankStatementRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TreasuryService>::get_internal_bank_statement(
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
                        let method = GetInternalBankStatementSvc(inner);
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
                        let mut response = http::Response::new(empty_body());
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
    impl<T> Clone for TreasuryServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "finance.treasury.v1.TreasuryService";
    impl<T> tonic::server::NamedService for TreasuryServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
