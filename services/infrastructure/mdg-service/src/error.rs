//! MDG 服务错误类型

use thiserror::Error;

/// MDG 服务错误
#[derive(Debug, Error)]
pub enum MdgError {
    #[error("实体不存在: {entity_type} {id}")]
    EntityNotFound {
        entity_type: String,
        id: String,
    },

    #[error("实体已存在: {entity_type} {id}")]
    EntityAlreadyExists {
        entity_type: String,
        id: String,
    },

    #[error("版本冲突: 期望 {expected}, 实际 {actual}")]
    VersionConflict {
        expected: i32,
        actual: i32,
    },

    #[error("验证失败: {0}")]
    ValidationError(String),

    #[error("租户不匹配: {0}")]
    TenantMismatch(String),

    #[error("层级验证失败: {0}")]
    HierarchyValidationFailed(String),

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Kafka 错误: {0}")]
    KafkaError(#[from] rdkafka::error::KafkaError),

    #[error("Redis 错误: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("配置错误: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("内部错误: {0}")]
    InternalError(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("重复数据检测到: {0}")]
    DuplicateDetected(String),
}

/// MDG 服务结果类型
pub type MdgResult<T> = Result<T, MdgError>;

impl From<MdgError> for tonic::Status {
    fn from(err: MdgError) -> Self {
        match err {
            MdgError::EntityNotFound { .. } => {
                tonic::Status::not_found(err.to_string())
            }
            MdgError::EntityAlreadyExists { .. } => {
                tonic::Status::already_exists(err.to_string())
            }
            MdgError::VersionConflict { .. } => {
                tonic::Status::aborted(err.to_string())
            }
            MdgError::ValidationError(_) => {
                tonic::Status::invalid_argument(err.to_string())
            }
            MdgError::TenantMismatch(_) => {
                tonic::Status::permission_denied(err.to_string())
            }
            MdgError::PermissionDenied(_) => {
                tonic::Status::permission_denied(err.to_string())
            }
            _ => tonic::Status::internal(err.to_string()),
        }
    }
}
