//! 服务模块
//!
//! gRPC 服务实现

pub mod journal_entry;

use tonic::Status;

// Health check service implementation
pub mod health {
    use tonic::{Request, Response};
    use tonic_health::pb::{health_check_response, HealthCheckRequest, HealthCheckResponse, ServingStatus};

    #[derive(Clone)]
    pub struct HealthService {
        status: ServingStatus,
    }

    impl HealthService {
        pub fn new() -> Self {
            Self {
                status: ServingStatus::Serving,
            }
        }

        pub fn set_status(&mut self, status: ServingStatus) {
            self.status = status;
        }
    }

    #[tonic::async_trait]
    impl tonic_health::HealthService for HealthService {
        async fn check(&self, _request: Request<HealthCheckRequest>) -> Result<Response<HealthCheckResponse>, Status> {
            Ok(Response::new(health_check_response::Status::Serving))
        }

        async fn watch(&self, _request: Request<HealthCheckRequest>) -> Result<Response<tonic::Streaming<HealthCheckResponse>>, Status> {
            Err(Status::unimplemented("watch not implemented"))
        }
    }
}
