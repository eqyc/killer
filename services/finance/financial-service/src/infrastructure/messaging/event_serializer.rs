//! 事件序列化器
//!
//! 提供事件的序列化和反序列化支持
//! 支持 JSON 和 Protobuf 两种格式

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

// =============================================================================
// 事件信封
// =============================================================================

/// 序列化后的事件信封
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEventEnvelope {
    /// 事件 ID
    pub event_id: Uuid,

    /// 事件名称
    pub event_type: String,

    /// 事件版本
    pub schema_version: u32,

    /// 聚合根类型
    pub aggregate_type: String,

    /// 聚合根 ID
    pub aggregate_id: String,

    /// 聚合根版本
    pub aggregate_version: i64,

    /// 租户 ID
    pub tenant_id: String,

    /// 发生时间
    pub occurred_at: DateTime<Utc>,

    /// 事件负载（JSON）
    pub payload: serde_json::Value,

    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}

impl SerializedEventEnvelope {
    /// 创建新的信封
    pub fn new(
        event_id: Uuid,
        event_type: String,
        schema_version: u32,
        aggregate_type: String,
        aggregate_id: String,
        aggregate_version: i64,
        tenant_id: String,
        occurred_at: DateTime<Utc>,
        payload: serde_json::Value,
        metadata: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            schema_version,
            aggregate_type,
            aggregate_id,
            aggregate_version,
            tenant_id,
            occurred_at,
            payload,
            metadata,
        }
    }

    /// 获取分区键（用于 Kafka）
    pub fn partition_key(&self) -> String {
        format!("{}-{}", self.tenant_id, self.aggregate_type)
    }
}

// =============================================================================
// 序列化格式
// =============================================================================

/// 序列化格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    Json,
    JsonPretty,
    Cbor,
    MessagePack,
}

/// 序列化错误
#[derive(thiserror::Error, Debug)]
pub enum SerializationError {
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),

    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),

    #[error("Unsupported format: {0:?}")]
    UnsupportedFormat(SerializationFormat),

    #[error("Schema version mismatch: expected {expected}, got {got}")]
    SchemaVersionMismatch { expected: u32, got: u32 },

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),
}

// =============================================================================
// 序列化器
// =============================================================================

/// 事件序列化器
pub struct EventSerializer {
    /// 默认序列化格式
    format: SerializationFormat,
    /// 忽略未知字段
    ignore_unknown_fields: bool,
}

impl Default for EventSerializer {
    fn default() -> Self {
        Self {
            format: SerializationFormat::Json,
            ignore_unknown_fields: true,
        }
    }
}

impl EventSerializer {
    /// 创建新的序列化器
    pub fn new(format: SerializationFormat) -> Self {
        Self {
            format,
            ignore_unknown_fields: true,
        }
    }

    /// 序列化事件信封为字节
    pub fn serialize_envelope(
        &self,
        envelope: &SerializedEventEnvelope,
    ) -> Result<Vec<u8>, SerializationError> {
        match self.format {
            SerializationFormat::Json => {
                serde_json::to_vec(envelope).map_err(|e| SerializationError::SerializationFailed(e.to_string()))
            }
            SerializationFormat::JsonPretty => {
                serde_json::to_vec_pretty(envelope).map_err(|e| SerializationError::SerializationFailed(e.to_string()))
            }
            SerializationFormat::Cbor => {
                serde_cbor::to_vec(envelope).map_err(|e| SerializationError::SerializationFailed(e.to_string()))
            }
            SerializationFormat::MessagePack => {
                rmp_serde::to_vec(envelope).map_err(|e| SerializationError::SerializationFailed(e.to_string()))
            }
        }
    }

    /// 从字节反序列化事件信封
    pub fn deserialize_envelope(
        &self,
        data: &[u8],
        expected_version: Option<u32>,
    ) -> Result<SerializedEventEnvelope, SerializationError> {
        let envelope: SerializedEventEnvelope = match self.format {
            SerializationFormat::Json => {
                if self.ignore_unknown_fields {
                    serde_json::from_slice(data)
                } else {
                    serde_json::from_slice(data)
                }
            }
            SerializationFormat::JsonPretty => serde_json::from_slice(data),
            SerializationFormat::Cbor => serde_cbor::from_slice(data),
            SerializationFormat::MessagePack => rmp_serde::from_slice(data),
        }
        .map_err(|e| SerializationError::DeserializationFailed(e.to_string()))?;

        // 验证 schema 版本
        if let Some(expected) = expected_version {
            if envelope.schema_version != expected {
                return Err(SerializationError::SchemaVersionMismatch {
                    expected,
                    got: envelope.schema_version,
                });
            }
        }

        Ok(envelope)
    }

    /// 序列化任意值为 JSON
    pub fn to_json_value<T: Serialize>(value: &T) -> Result<serde_json::Value, SerializationError> {
        serde_json::to_value(value)
            .map_err(|e| SerializationError::SerializationFailed(e.to_string()))
    }

    /// 从 JSON 反序列化
    pub fn from_json_value<T: DeserializeOwned>(value: &serde_json::Value) -> Result<T, SerializationError>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(value.clone())
            .map_err(|e| SerializationError::DeserializationFailed(e.to_string()))
    }
}

// =============================================================================
// 事件演进支持
// =============================================================================

/// 事件演进配置
#[derive(Debug, Clone)]
pub struct EventEvolutionConfig {
    /// 当前 schema 版本
    pub current_version: u32,
    /// 最大保留历史版本数
    pub max_history_versions: u32,
    /// 演化规则
    pub evolution_rules: Vec<EvolutionRule>,
}

/// 演化规则
#[derive(Debug, Clone)]
pub struct EvolutionRule {
    /// 源版本
    pub from_version: u32,
    /// 目标版本
    pub to_version: u32,
    /// 演化函数
    pub transform: Box<dyn Fn(serde_json::Value) -> Result<serde_json::Value, SerializationError> + Send + Sync>,
}

impl EventEvolutionConfig {
    /// 演化事件到当前版本
    pub fn evolve(
        &self,
        payload: serde_json::Value,
        from_version: u32,
    ) -> Result<serde_json::Value, SerializationError> {
        let mut current = payload;
        let mut version = from_version;

        while version < self.current_version {
            let rule = self
                .evolution_rules
                .iter()
                .find(|r| r.from_version == version)
                .ok_or_else(|| {
                    SerializationError::InvalidPayload(format!(
                        "No evolution rule from version {} to {}",
                        version, version + 1
                    ))
                })?;

            current = (rule.transform)(current)?;
            version += 1;
        }

        Ok(current)
    }
}

// =============================================================================
// 便利函数
// =============================================================================

/// 快速创建 JSON 序列化器
pub fn json_serializer() -> EventSerializer {
    EventSerializer::new(SerializationFormat::Json)
}

/// 快速创建带pretty JSON 序列化器
pub fn json_pretty_serializer() -> EventSerializer {
    EventSerializer::new(SerializationFormat::JsonPretty)
}
