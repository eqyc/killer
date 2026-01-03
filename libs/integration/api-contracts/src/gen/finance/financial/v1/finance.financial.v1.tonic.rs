// @generated
/// Generated client implementations.
pub mod financial_service_client {
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
    pub struct FinancialServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FinancialServiceClient<tonic::transport::Channel> {
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
    impl<T> FinancialServiceClient<T>
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
        ) -> FinancialServiceClient<InterceptedService<T, F>>
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
            FinancialServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_gl_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGlAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGlAccountResponse>,
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
                "/finance.financial.v1.FinancialService/GetGLAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetGLAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_gl_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGlAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGlAccountsResponse>,
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
                "/finance.financial.v1.FinancialService/ListGLAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ListGLAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_journal_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateJournalEntryResponse>,
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
                "/finance.financial.v1.FinancialService/CreateJournalEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "CreateJournalEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_journal_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetJournalEntryResponse>,
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
                "/finance.financial.v1.FinancialService/GetJournalEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetJournalEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_journal_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJournalEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListJournalEntriesResponse>,
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
                "/finance.financial.v1.FinancialService/ListJournalEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ListJournalEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_journal_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::PostJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostJournalEntryResponse>,
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
                "/finance.financial.v1.FinancialService/PostJournalEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostJournalEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reverse_journal_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::ReverseJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReverseJournalEntryResponse>,
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
                "/finance.financial.v1.FinancialService/ReverseJournalEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ReverseJournalEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_create_journal_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateJournalEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateJournalEntriesResponse>,
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
                "/finance.financial.v1.FinancialService/BatchCreateJournalEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "BatchCreateJournalEntries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_journal_entry_with_template(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateJournalEntryWithTemplateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreateJournalEntryWithTemplateResponse>,
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
                "/finance.financial.v1.FinancialService/CreateJournalEntryWithTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "CreateJournalEntryWithTemplate",
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
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/finance.financial.v1.FinancialService/GetAccountBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetAccountBalance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account_balances(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountBalancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountBalancesResponse>,
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
                "/finance.financial.v1.FinancialService/GetAccountBalances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetAccountBalances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_trial_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateTrialBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateTrialBalanceResponse>,
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
                "/finance.financial.v1.FinancialService/GenerateTrialBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GenerateTrialBalance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
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
                "/finance.financial.v1.FinancialService/GetCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_customers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomersResponse>,
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
                "/finance.financial.v1.FinancialService/ListCustomers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ListCustomers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCustomerResponse>,
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
                "/finance.financial.v1.FinancialService/CreateCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "CreateCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomerResponse>,
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
                "/finance.financial.v1.FinancialService/UpdateCustomer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "UpdateCustomer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_receivable_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::PostReceivableEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostReceivableEntryResponse>,
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
                "/finance.financial.v1.FinancialService/PostReceivableEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostReceivableEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_incoming_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::PostIncomingPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostIncomingPaymentResponse>,
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
                "/finance.financial.v1.FinancialService/PostIncomingPayment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostIncomingPayment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn clear_customer_open_item(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearCustomerOpenItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearCustomerOpenItemResponse>,
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
                "/finance.financial.v1.FinancialService/ClearCustomerOpenItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ClearCustomerOpenItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn send_customer_dunning(
            &mut self,
            request: impl tonic::IntoRequest<super::SendCustomerDunningRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendCustomerDunningResponse>,
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
                "/finance.financial.v1.FinancialService/SendCustomerDunning",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "SendCustomerDunning",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn calculate_exchange_difference(
            &mut self,
            request: impl tonic::IntoRequest<super::CalculateExchangeDifferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CalculateExchangeDifferenceResponse>,
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
                "/finance.financial.v1.FinancialService/CalculateExchangeDifference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "CalculateExchangeDifference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_vendor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVendorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVendorResponse>,
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
                "/finance.financial.v1.FinancialService/GetVendor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.financial.v1.FinancialService", "GetVendor"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_vendors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVendorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVendorsResponse>,
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
                "/finance.financial.v1.FinancialService/ListVendors",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ListVendors",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_vendor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVendorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateVendorResponse>,
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
                "/finance.financial.v1.FinancialService/CreateVendor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "CreateVendor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_vendor(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVendorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateVendorResponse>,
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
                "/finance.financial.v1.FinancialService/UpdateVendor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "UpdateVendor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_payable_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::PostPayableEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostPayableEntryResponse>,
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
                "/finance.financial.v1.FinancialService/PostPayableEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostPayableEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_outgoing_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOutgoingPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostOutgoingPaymentResponse>,
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
                "/finance.financial.v1.FinancialService/PostOutgoingPayment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostOutgoingPayment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn clear_vendor_open_item(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearVendorOpenItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearVendorOpenItemResponse>,
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
                "/finance.financial.v1.FinancialService/ClearVendorOpenItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ClearVendorOpenItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_payment_block(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPaymentBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetPaymentBlockResponse>,
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
                "/finance.financial.v1.FinancialService/SetPaymentBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "SetPaymentBlock",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_fixed_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFixedAssetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFixedAssetResponse>,
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
                "/finance.financial.v1.FinancialService/GetFixedAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetFixedAsset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_fixed_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFixedAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFixedAssetsResponse>,
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
                "/finance.financial.v1.FinancialService/ListFixedAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "ListFixedAssets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_fixed_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFixedAssetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFixedAssetResponse>,
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
                "/finance.financial.v1.FinancialService/CreateFixedAsset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "CreateFixedAsset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_asset_acquisition(
            &mut self,
            request: impl tonic::IntoRequest<super::PostAssetAcquisitionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetAcquisitionResponse>,
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
                "/finance.financial.v1.FinancialService/PostAssetAcquisition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostAssetAcquisition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_asset_depreciation(
            &mut self,
            request: impl tonic::IntoRequest<super::PostAssetDepreciationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetDepreciationResponse>,
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
                "/finance.financial.v1.FinancialService/PostAssetDepreciation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostAssetDepreciation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_asset_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::PostAssetTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetTransferResponse>,
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
                "/finance.financial.v1.FinancialService/PostAssetTransfer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostAssetTransfer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_asset_retirement(
            &mut self,
            request: impl tonic::IntoRequest<super::PostAssetRetirementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetRetirementResponse>,
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
                "/finance.financial.v1.FinancialService/PostAssetRetirement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostAssetRetirement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_asset_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAssetValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAssetValueResponse>,
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
                "/finance.financial.v1.FinancialService/GetAssetValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetAssetValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_bank(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBankRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBankResponse>,
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
                "/finance.financial.v1.FinancialService/GetBank",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.financial.v1.FinancialService", "GetBank"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_banks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBanksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBanksResponse>,
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
                "/finance.financial.v1.FinancialService/ListBanks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("finance.financial.v1.FinancialService", "ListBanks"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_bank_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::PostBankPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostBankPaymentResponse>,
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
                "/finance.financial.v1.FinancialService/PostBankPayment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostBankPayment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_bank_receipt(
            &mut self,
            request: impl tonic::IntoRequest<super::PostBankReceiptRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostBankReceiptResponse>,
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
                "/finance.financial.v1.FinancialService/PostBankReceipt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostBankReceipt",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_bank_reconciliation(
            &mut self,
            request: impl tonic::IntoRequest<super::PostBankReconciliationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostBankReconciliationResponse>,
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
                "/finance.financial.v1.FinancialService/PostBankReconciliation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostBankReconciliation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_bank_account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBankAccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBankAccountBalanceResponse>,
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
                "/finance.financial.v1.FinancialService/GetBankAccountBalance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GetBankAccountBalance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn open_fiscal_period(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenFiscalPeriodRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenFiscalPeriodResponse>,
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
                "/finance.financial.v1.FinancialService/OpenFiscalPeriod",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "OpenFiscalPeriod",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn close_fiscal_period(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseFiscalPeriodRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloseFiscalPeriodResponse>,
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
                "/finance.financial.v1.FinancialService/CloseFiscalPeriod",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "CloseFiscalPeriod",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn post_period_closing_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::PostPeriodClosingEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostPeriodClosingEntryResponse>,
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
                "/finance.financial.v1.FinancialService/PostPeriodClosingEntry",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "PostPeriodClosingEntry",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn generate_gl_account_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateGlAccountReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateGlAccountReportResponse>,
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
                "/finance.financial.v1.FinancialService/GenerateGLAccountReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "finance.financial.v1.FinancialService",
                        "GenerateGLAccountReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod financial_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with FinancialServiceServer.
    #[async_trait]
    pub trait FinancialService: std::marker::Send + std::marker::Sync + 'static {
        async fn get_gl_account(
            &self,
            request: tonic::Request<super::GetGlAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGlAccountResponse>,
            tonic::Status,
        >;
        async fn list_gl_accounts(
            &self,
            request: tonic::Request<super::ListGlAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGlAccountsResponse>,
            tonic::Status,
        >;
        async fn create_journal_entry(
            &self,
            request: tonic::Request<super::CreateJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateJournalEntryResponse>,
            tonic::Status,
        >;
        async fn get_journal_entry(
            &self,
            request: tonic::Request<super::GetJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetJournalEntryResponse>,
            tonic::Status,
        >;
        async fn list_journal_entries(
            &self,
            request: tonic::Request<super::ListJournalEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListJournalEntriesResponse>,
            tonic::Status,
        >;
        async fn post_journal_entry(
            &self,
            request: tonic::Request<super::PostJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostJournalEntryResponse>,
            tonic::Status,
        >;
        async fn reverse_journal_entry(
            &self,
            request: tonic::Request<super::ReverseJournalEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReverseJournalEntryResponse>,
            tonic::Status,
        >;
        async fn batch_create_journal_entries(
            &self,
            request: tonic::Request<super::BatchCreateJournalEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateJournalEntriesResponse>,
            tonic::Status,
        >;
        async fn create_journal_entry_with_template(
            &self,
            request: tonic::Request<super::CreateJournalEntryWithTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateJournalEntryWithTemplateResponse>,
            tonic::Status,
        >;
        async fn get_account_balance(
            &self,
            request: tonic::Request<super::GetAccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountBalanceResponse>,
            tonic::Status,
        >;
        async fn get_account_balances(
            &self,
            request: tonic::Request<super::GetAccountBalancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountBalancesResponse>,
            tonic::Status,
        >;
        async fn generate_trial_balance(
            &self,
            request: tonic::Request<super::GenerateTrialBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateTrialBalanceResponse>,
            tonic::Status,
        >;
        async fn get_customer(
            &self,
            request: tonic::Request<super::GetCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCustomerResponse>,
            tonic::Status,
        >;
        async fn list_customers(
            &self,
            request: tonic::Request<super::ListCustomersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomersResponse>,
            tonic::Status,
        >;
        async fn create_customer(
            &self,
            request: tonic::Request<super::CreateCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCustomerResponse>,
            tonic::Status,
        >;
        async fn update_customer(
            &self,
            request: tonic::Request<super::UpdateCustomerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCustomerResponse>,
            tonic::Status,
        >;
        async fn post_receivable_entry(
            &self,
            request: tonic::Request<super::PostReceivableEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostReceivableEntryResponse>,
            tonic::Status,
        >;
        async fn post_incoming_payment(
            &self,
            request: tonic::Request<super::PostIncomingPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostIncomingPaymentResponse>,
            tonic::Status,
        >;
        async fn clear_customer_open_item(
            &self,
            request: tonic::Request<super::ClearCustomerOpenItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearCustomerOpenItemResponse>,
            tonic::Status,
        >;
        async fn send_customer_dunning(
            &self,
            request: tonic::Request<super::SendCustomerDunningRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SendCustomerDunningResponse>,
            tonic::Status,
        >;
        async fn calculate_exchange_difference(
            &self,
            request: tonic::Request<super::CalculateExchangeDifferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CalculateExchangeDifferenceResponse>,
            tonic::Status,
        >;
        async fn get_vendor(
            &self,
            request: tonic::Request<super::GetVendorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVendorResponse>,
            tonic::Status,
        >;
        async fn list_vendors(
            &self,
            request: tonic::Request<super::ListVendorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVendorsResponse>,
            tonic::Status,
        >;
        async fn create_vendor(
            &self,
            request: tonic::Request<super::CreateVendorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateVendorResponse>,
            tonic::Status,
        >;
        async fn update_vendor(
            &self,
            request: tonic::Request<super::UpdateVendorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateVendorResponse>,
            tonic::Status,
        >;
        async fn post_payable_entry(
            &self,
            request: tonic::Request<super::PostPayableEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostPayableEntryResponse>,
            tonic::Status,
        >;
        async fn post_outgoing_payment(
            &self,
            request: tonic::Request<super::PostOutgoingPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostOutgoingPaymentResponse>,
            tonic::Status,
        >;
        async fn clear_vendor_open_item(
            &self,
            request: tonic::Request<super::ClearVendorOpenItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearVendorOpenItemResponse>,
            tonic::Status,
        >;
        async fn set_payment_block(
            &self,
            request: tonic::Request<super::SetPaymentBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetPaymentBlockResponse>,
            tonic::Status,
        >;
        async fn get_fixed_asset(
            &self,
            request: tonic::Request<super::GetFixedAssetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFixedAssetResponse>,
            tonic::Status,
        >;
        async fn list_fixed_assets(
            &self,
            request: tonic::Request<super::ListFixedAssetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFixedAssetsResponse>,
            tonic::Status,
        >;
        async fn create_fixed_asset(
            &self,
            request: tonic::Request<super::CreateFixedAssetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFixedAssetResponse>,
            tonic::Status,
        >;
        async fn post_asset_acquisition(
            &self,
            request: tonic::Request<super::PostAssetAcquisitionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetAcquisitionResponse>,
            tonic::Status,
        >;
        async fn post_asset_depreciation(
            &self,
            request: tonic::Request<super::PostAssetDepreciationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetDepreciationResponse>,
            tonic::Status,
        >;
        async fn post_asset_transfer(
            &self,
            request: tonic::Request<super::PostAssetTransferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetTransferResponse>,
            tonic::Status,
        >;
        async fn post_asset_retirement(
            &self,
            request: tonic::Request<super::PostAssetRetirementRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostAssetRetirementResponse>,
            tonic::Status,
        >;
        async fn get_asset_value(
            &self,
            request: tonic::Request<super::GetAssetValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAssetValueResponse>,
            tonic::Status,
        >;
        async fn get_bank(
            &self,
            request: tonic::Request<super::GetBankRequest>,
        ) -> std::result::Result<tonic::Response<super::GetBankResponse>, tonic::Status>;
        async fn list_banks(
            &self,
            request: tonic::Request<super::ListBanksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBanksResponse>,
            tonic::Status,
        >;
        async fn post_bank_payment(
            &self,
            request: tonic::Request<super::PostBankPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostBankPaymentResponse>,
            tonic::Status,
        >;
        async fn post_bank_receipt(
            &self,
            request: tonic::Request<super::PostBankReceiptRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostBankReceiptResponse>,
            tonic::Status,
        >;
        async fn post_bank_reconciliation(
            &self,
            request: tonic::Request<super::PostBankReconciliationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostBankReconciliationResponse>,
            tonic::Status,
        >;
        async fn get_bank_account_balance(
            &self,
            request: tonic::Request<super::GetBankAccountBalanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBankAccountBalanceResponse>,
            tonic::Status,
        >;
        async fn open_fiscal_period(
            &self,
            request: tonic::Request<super::OpenFiscalPeriodRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenFiscalPeriodResponse>,
            tonic::Status,
        >;
        async fn close_fiscal_period(
            &self,
            request: tonic::Request<super::CloseFiscalPeriodRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloseFiscalPeriodResponse>,
            tonic::Status,
        >;
        async fn post_period_closing_entry(
            &self,
            request: tonic::Request<super::PostPeriodClosingEntryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostPeriodClosingEntryResponse>,
            tonic::Status,
        >;
        async fn generate_gl_account_report(
            &self,
            request: tonic::Request<super::GenerateGlAccountReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateGlAccountReportResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct FinancialServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> FinancialServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FinancialServiceServer<T>
    where
        T: FinancialService,
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
                "/finance.financial.v1.FinancialService/GetGLAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetGLAccountSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetGlAccountRequest>
                    for GetGLAccountSvc<T> {
                        type Response = super::GetGlAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGlAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_gl_account(&inner, request)
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
                        let method = GetGLAccountSvc(inner);
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
                "/finance.financial.v1.FinancialService/ListGLAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct ListGLAccountsSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ListGlAccountsRequest>
                    for ListGLAccountsSvc<T> {
                        type Response = super::ListGlAccountsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGlAccountsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::list_gl_accounts(&inner, request)
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
                        let method = ListGLAccountsSvc(inner);
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
                "/finance.financial.v1.FinancialService/CreateJournalEntry" => {
                    #[allow(non_camel_case_types)]
                    struct CreateJournalEntrySvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::CreateJournalEntryRequest>
                    for CreateJournalEntrySvc<T> {
                        type Response = super::CreateJournalEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateJournalEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::create_journal_entry(
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
                        let method = CreateJournalEntrySvc(inner);
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
                "/finance.financial.v1.FinancialService/GetJournalEntry" => {
                    #[allow(non_camel_case_types)]
                    struct GetJournalEntrySvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetJournalEntryRequest>
                    for GetJournalEntrySvc<T> {
                        type Response = super::GetJournalEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJournalEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_journal_entry(&inner, request)
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
                        let method = GetJournalEntrySvc(inner);
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
                "/finance.financial.v1.FinancialService/ListJournalEntries" => {
                    #[allow(non_camel_case_types)]
                    struct ListJournalEntriesSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ListJournalEntriesRequest>
                    for ListJournalEntriesSvc<T> {
                        type Response = super::ListJournalEntriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListJournalEntriesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::list_journal_entries(
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
                        let method = ListJournalEntriesSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostJournalEntry" => {
                    #[allow(non_camel_case_types)]
                    struct PostJournalEntrySvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostJournalEntryRequest>
                    for PostJournalEntrySvc<T> {
                        type Response = super::PostJournalEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostJournalEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_journal_entry(&inner, request)
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
                        let method = PostJournalEntrySvc(inner);
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
                "/finance.financial.v1.FinancialService/ReverseJournalEntry" => {
                    #[allow(non_camel_case_types)]
                    struct ReverseJournalEntrySvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ReverseJournalEntryRequest>
                    for ReverseJournalEntrySvc<T> {
                        type Response = super::ReverseJournalEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReverseJournalEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::reverse_journal_entry(
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
                        let method = ReverseJournalEntrySvc(inner);
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
                "/finance.financial.v1.FinancialService/BatchCreateJournalEntries" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateJournalEntriesSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<
                        super::BatchCreateJournalEntriesRequest,
                    > for BatchCreateJournalEntriesSvc<T> {
                        type Response = super::BatchCreateJournalEntriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchCreateJournalEntriesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::batch_create_journal_entries(
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
                        let method = BatchCreateJournalEntriesSvc(inner);
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
                "/finance.financial.v1.FinancialService/CreateJournalEntryWithTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct CreateJournalEntryWithTemplateSvc<T: FinancialService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<
                        super::CreateJournalEntryWithTemplateRequest,
                    > for CreateJournalEntryWithTemplateSvc<T> {
                        type Response = super::CreateJournalEntryWithTemplateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateJournalEntryWithTemplateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::create_journal_entry_with_template(
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
                        let method = CreateJournalEntryWithTemplateSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetAccountBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountBalanceSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
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
                                <T as FinancialService>::get_account_balance(
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
                        let method = GetAccountBalanceSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetAccountBalances" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountBalancesSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetAccountBalancesRequest>
                    for GetAccountBalancesSvc<T> {
                        type Response = super::GetAccountBalancesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountBalancesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_account_balances(
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
                        let method = GetAccountBalancesSvc(inner);
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
                "/finance.financial.v1.FinancialService/GenerateTrialBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateTrialBalanceSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GenerateTrialBalanceRequest>
                    for GenerateTrialBalanceSvc<T> {
                        type Response = super::GenerateTrialBalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateTrialBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::generate_trial_balance(
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
                        let method = GenerateTrialBalanceSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct GetCustomerSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetCustomerRequest>
                    for GetCustomerSvc<T> {
                        type Response = super::GetCustomerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCustomerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_customer(&inner, request).await
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
                        let method = GetCustomerSvc(inner);
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
                "/finance.financial.v1.FinancialService/ListCustomers" => {
                    #[allow(non_camel_case_types)]
                    struct ListCustomersSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ListCustomersRequest>
                    for ListCustomersSvc<T> {
                        type Response = super::ListCustomersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCustomersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::list_customers(&inner, request)
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
                        let method = ListCustomersSvc(inner);
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
                "/finance.financial.v1.FinancialService/CreateCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCustomerSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::CreateCustomerRequest>
                    for CreateCustomerSvc<T> {
                        type Response = super::CreateCustomerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCustomerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::create_customer(&inner, request)
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
                        let method = CreateCustomerSvc(inner);
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
                "/finance.financial.v1.FinancialService/UpdateCustomer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCustomerSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::UpdateCustomerRequest>
                    for UpdateCustomerSvc<T> {
                        type Response = super::UpdateCustomerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCustomerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::update_customer(&inner, request)
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
                        let method = UpdateCustomerSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostReceivableEntry" => {
                    #[allow(non_camel_case_types)]
                    struct PostReceivableEntrySvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostReceivableEntryRequest>
                    for PostReceivableEntrySvc<T> {
                        type Response = super::PostReceivableEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostReceivableEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_receivable_entry(
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
                        let method = PostReceivableEntrySvc(inner);
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
                "/finance.financial.v1.FinancialService/PostIncomingPayment" => {
                    #[allow(non_camel_case_types)]
                    struct PostIncomingPaymentSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostIncomingPaymentRequest>
                    for PostIncomingPaymentSvc<T> {
                        type Response = super::PostIncomingPaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostIncomingPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_incoming_payment(
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
                        let method = PostIncomingPaymentSvc(inner);
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
                "/finance.financial.v1.FinancialService/ClearCustomerOpenItem" => {
                    #[allow(non_camel_case_types)]
                    struct ClearCustomerOpenItemSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ClearCustomerOpenItemRequest>
                    for ClearCustomerOpenItemSvc<T> {
                        type Response = super::ClearCustomerOpenItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearCustomerOpenItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::clear_customer_open_item(
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
                        let method = ClearCustomerOpenItemSvc(inner);
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
                "/finance.financial.v1.FinancialService/SendCustomerDunning" => {
                    #[allow(non_camel_case_types)]
                    struct SendCustomerDunningSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::SendCustomerDunningRequest>
                    for SendCustomerDunningSvc<T> {
                        type Response = super::SendCustomerDunningResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendCustomerDunningRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::send_customer_dunning(
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
                        let method = SendCustomerDunningSvc(inner);
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
                "/finance.financial.v1.FinancialService/CalculateExchangeDifference" => {
                    #[allow(non_camel_case_types)]
                    struct CalculateExchangeDifferenceSvc<T: FinancialService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<
                        super::CalculateExchangeDifferenceRequest,
                    > for CalculateExchangeDifferenceSvc<T> {
                        type Response = super::CalculateExchangeDifferenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CalculateExchangeDifferenceRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::calculate_exchange_difference(
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
                        let method = CalculateExchangeDifferenceSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetVendor" => {
                    #[allow(non_camel_case_types)]
                    struct GetVendorSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetVendorRequest>
                    for GetVendorSvc<T> {
                        type Response = super::GetVendorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVendorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_vendor(&inner, request).await
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
                        let method = GetVendorSvc(inner);
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
                "/finance.financial.v1.FinancialService/ListVendors" => {
                    #[allow(non_camel_case_types)]
                    struct ListVendorsSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ListVendorsRequest>
                    for ListVendorsSvc<T> {
                        type Response = super::ListVendorsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVendorsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::list_vendors(&inner, request).await
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
                        let method = ListVendorsSvc(inner);
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
                "/finance.financial.v1.FinancialService/CreateVendor" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVendorSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::CreateVendorRequest>
                    for CreateVendorSvc<T> {
                        type Response = super::CreateVendorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateVendorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::create_vendor(&inner, request)
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
                        let method = CreateVendorSvc(inner);
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
                "/finance.financial.v1.FinancialService/UpdateVendor" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVendorSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::UpdateVendorRequest>
                    for UpdateVendorSvc<T> {
                        type Response = super::UpdateVendorResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateVendorRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::update_vendor(&inner, request)
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
                        let method = UpdateVendorSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostPayableEntry" => {
                    #[allow(non_camel_case_types)]
                    struct PostPayableEntrySvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostPayableEntryRequest>
                    for PostPayableEntrySvc<T> {
                        type Response = super::PostPayableEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostPayableEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_payable_entry(&inner, request)
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
                        let method = PostPayableEntrySvc(inner);
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
                "/finance.financial.v1.FinancialService/PostOutgoingPayment" => {
                    #[allow(non_camel_case_types)]
                    struct PostOutgoingPaymentSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostOutgoingPaymentRequest>
                    for PostOutgoingPaymentSvc<T> {
                        type Response = super::PostOutgoingPaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostOutgoingPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_outgoing_payment(
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
                        let method = PostOutgoingPaymentSvc(inner);
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
                "/finance.financial.v1.FinancialService/ClearVendorOpenItem" => {
                    #[allow(non_camel_case_types)]
                    struct ClearVendorOpenItemSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ClearVendorOpenItemRequest>
                    for ClearVendorOpenItemSvc<T> {
                        type Response = super::ClearVendorOpenItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearVendorOpenItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::clear_vendor_open_item(
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
                        let method = ClearVendorOpenItemSvc(inner);
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
                "/finance.financial.v1.FinancialService/SetPaymentBlock" => {
                    #[allow(non_camel_case_types)]
                    struct SetPaymentBlockSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::SetPaymentBlockRequest>
                    for SetPaymentBlockSvc<T> {
                        type Response = super::SetPaymentBlockResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPaymentBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::set_payment_block(&inner, request)
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
                        let method = SetPaymentBlockSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetFixedAsset" => {
                    #[allow(non_camel_case_types)]
                    struct GetFixedAssetSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetFixedAssetRequest>
                    for GetFixedAssetSvc<T> {
                        type Response = super::GetFixedAssetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFixedAssetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_fixed_asset(&inner, request)
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
                        let method = GetFixedAssetSvc(inner);
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
                "/finance.financial.v1.FinancialService/ListFixedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct ListFixedAssetsSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ListFixedAssetsRequest>
                    for ListFixedAssetsSvc<T> {
                        type Response = super::ListFixedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFixedAssetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::list_fixed_assets(&inner, request)
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
                        let method = ListFixedAssetsSvc(inner);
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
                "/finance.financial.v1.FinancialService/CreateFixedAsset" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFixedAssetSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::CreateFixedAssetRequest>
                    for CreateFixedAssetSvc<T> {
                        type Response = super::CreateFixedAssetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFixedAssetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::create_fixed_asset(&inner, request)
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
                        let method = CreateFixedAssetSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostAssetAcquisition" => {
                    #[allow(non_camel_case_types)]
                    struct PostAssetAcquisitionSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostAssetAcquisitionRequest>
                    for PostAssetAcquisitionSvc<T> {
                        type Response = super::PostAssetAcquisitionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostAssetAcquisitionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_asset_acquisition(
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
                        let method = PostAssetAcquisitionSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostAssetDepreciation" => {
                    #[allow(non_camel_case_types)]
                    struct PostAssetDepreciationSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostAssetDepreciationRequest>
                    for PostAssetDepreciationSvc<T> {
                        type Response = super::PostAssetDepreciationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostAssetDepreciationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_asset_depreciation(
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
                        let method = PostAssetDepreciationSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostAssetTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct PostAssetTransferSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostAssetTransferRequest>
                    for PostAssetTransferSvc<T> {
                        type Response = super::PostAssetTransferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostAssetTransferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_asset_transfer(
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
                        let method = PostAssetTransferSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostAssetRetirement" => {
                    #[allow(non_camel_case_types)]
                    struct PostAssetRetirementSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostAssetRetirementRequest>
                    for PostAssetRetirementSvc<T> {
                        type Response = super::PostAssetRetirementResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostAssetRetirementRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_asset_retirement(
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
                        let method = PostAssetRetirementSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetAssetValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetAssetValueSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetAssetValueRequest>
                    for GetAssetValueSvc<T> {
                        type Response = super::GetAssetValueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAssetValueRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_asset_value(&inner, request)
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
                        let method = GetAssetValueSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetBank" => {
                    #[allow(non_camel_case_types)]
                    struct GetBankSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetBankRequest>
                    for GetBankSvc<T> {
                        type Response = super::GetBankResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBankRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_bank(&inner, request).await
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
                        let method = GetBankSvc(inner);
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
                "/finance.financial.v1.FinancialService/ListBanks" => {
                    #[allow(non_camel_case_types)]
                    struct ListBanksSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::ListBanksRequest>
                    for ListBanksSvc<T> {
                        type Response = super::ListBanksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBanksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::list_banks(&inner, request).await
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
                        let method = ListBanksSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostBankPayment" => {
                    #[allow(non_camel_case_types)]
                    struct PostBankPaymentSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostBankPaymentRequest>
                    for PostBankPaymentSvc<T> {
                        type Response = super::PostBankPaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostBankPaymentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_bank_payment(&inner, request)
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
                        let method = PostBankPaymentSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostBankReceipt" => {
                    #[allow(non_camel_case_types)]
                    struct PostBankReceiptSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostBankReceiptRequest>
                    for PostBankReceiptSvc<T> {
                        type Response = super::PostBankReceiptResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostBankReceiptRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_bank_receipt(&inner, request)
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
                        let method = PostBankReceiptSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostBankReconciliation" => {
                    #[allow(non_camel_case_types)]
                    struct PostBankReconciliationSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostBankReconciliationRequest>
                    for PostBankReconciliationSvc<T> {
                        type Response = super::PostBankReconciliationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostBankReconciliationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_bank_reconciliation(
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
                        let method = PostBankReconciliationSvc(inner);
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
                "/finance.financial.v1.FinancialService/GetBankAccountBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetBankAccountBalanceSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GetBankAccountBalanceRequest>
                    for GetBankAccountBalanceSvc<T> {
                        type Response = super::GetBankAccountBalanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBankAccountBalanceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::get_bank_account_balance(
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
                        let method = GetBankAccountBalanceSvc(inner);
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
                "/finance.financial.v1.FinancialService/OpenFiscalPeriod" => {
                    #[allow(non_camel_case_types)]
                    struct OpenFiscalPeriodSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::OpenFiscalPeriodRequest>
                    for OpenFiscalPeriodSvc<T> {
                        type Response = super::OpenFiscalPeriodResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenFiscalPeriodRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::open_fiscal_period(&inner, request)
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
                        let method = OpenFiscalPeriodSvc(inner);
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
                "/finance.financial.v1.FinancialService/CloseFiscalPeriod" => {
                    #[allow(non_camel_case_types)]
                    struct CloseFiscalPeriodSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::CloseFiscalPeriodRequest>
                    for CloseFiscalPeriodSvc<T> {
                        type Response = super::CloseFiscalPeriodResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloseFiscalPeriodRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::close_fiscal_period(
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
                        let method = CloseFiscalPeriodSvc(inner);
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
                "/finance.financial.v1.FinancialService/PostPeriodClosingEntry" => {
                    #[allow(non_camel_case_types)]
                    struct PostPeriodClosingEntrySvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::PostPeriodClosingEntryRequest>
                    for PostPeriodClosingEntrySvc<T> {
                        type Response = super::PostPeriodClosingEntryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PostPeriodClosingEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::post_period_closing_entry(
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
                        let method = PostPeriodClosingEntrySvc(inner);
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
                "/finance.financial.v1.FinancialService/GenerateGLAccountReport" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateGLAccountReportSvc<T: FinancialService>(pub Arc<T>);
                    impl<
                        T: FinancialService,
                    > tonic::server::UnaryService<super::GenerateGlAccountReportRequest>
                    for GenerateGLAccountReportSvc<T> {
                        type Response = super::GenerateGlAccountReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GenerateGlAccountReportRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FinancialService>::generate_gl_account_report(
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
                        let method = GenerateGLAccountReportSvc(inner);
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
    impl<T> Clone for FinancialServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "finance.financial.v1.FinancialService";
    impl<T> tonic::server::NamedService for FinancialServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
