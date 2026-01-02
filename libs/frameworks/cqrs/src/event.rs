//! 领域事件系统
//!
//! 定义领域事件、事件信封和事件总线

use crate::error::AppError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

/// 领域事件标记 trait
///
/// 所有领域事件必须实现此 trait
pub trait DomainEvent:
    Debug + Clone + Serialize + DeserializeOwned + Send + Sync + 'static
{
    /// 事件名称（用于序列化和路由）
    fn event_name() -> &'static str;

    /// 事件版本（用于事件演化）
    fn event_version() -> u32 {
        1
    }
}

/// 事件信封
///
/// 包含事件元数据和事件负载
pub struct EventEnvelope<E: DomainEvent> {
    /// 事件 ID
    pub event_id: Uuid,

    /// 事件名称
    pub event_name: String,

    /// 事件版本
    pub event_version: u32,

    /// 聚合根 ID
    pub aggregate_id: String,

    /// 聚合根类型
    pub aggregate_type: String,

    /// 聚合根版本（用于乐观锁）
    pub aggregate_version: i64,

    /// 租户 ID
    pub tenant_id: String,

    /// 发生时间
    pub occurred_at: DateTime<Utc>,

    /// 事件负载
    pub payload: E,

    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
}

impl<E: DomainEvent> Debug for EventEnvelope<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventEnvelope")
            .field("event_id", &self.event_id)
            .field("event_name", &self.event_name)
            .field("aggregate_id", &self.aggregate_id)
            .field("aggregate_type", &self.aggregate_type)
            .finish()
    }
}

impl<E: DomainEvent> Clone for EventEnvelope<E> {
    fn clone(&self) -> Self {
        Self {
            event_id: self.event_id,
            event_name: self.event_name.clone(),
            event_version: self.event_version,
            aggregate_id: self.aggregate_id.clone(),
            aggregate_type: self.aggregate_type.clone(),
            aggregate_version: self.aggregate_version,
            tenant_id: self.tenant_id.clone(),
            occurred_at: self.occurred_at,
            payload: self.payload.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<E: DomainEvent> Serialize for EventEnvelope<E> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("EventEnvelope", 11)?;
        state.serialize_field("event_id", &self.event_id)?;
        state.serialize_field("event_name", &self.event_name)?;
        state.serialize_field("event_version", &self.event_version)?;
        state.serialize_field("aggregate_id", &self.aggregate_id)?;
        state.serialize_field("aggregate_type", &self.aggregate_type)?;
        state.serialize_field("aggregate_version", &self.aggregate_version)?;
        state.serialize_field("tenant_id", &self.tenant_id)?;
        state.serialize_field("occurred_at", &self.occurred_at)?;
        state.serialize_field("payload", &self.payload)?;
        state.serialize_field("metadata", &self.metadata)?;
        state.end()
    }
}

impl<'de, E: DomainEvent + DeserializeOwned> Deserialize<'de> for EventEnvelope<E> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;
        use std::fmt;

        struct EventEnvelopeVisitor<E> {
            _phantom: std::marker::PhantomData<E>,
        }

        impl<'de, E: DomainEvent + DeserializeOwned> Visitor<'de> for EventEnvelopeVisitor<E> {
            type Value = EventEnvelope<E>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("EventEnvelope")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventEnvelope<E>, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut event_id = None;
                let mut event_name = None;
                let mut event_version = None;
                let mut aggregate_id = None;
                let mut aggregate_type = None;
                let mut aggregate_version = None;
                let mut tenant_id = None;
                let mut occurred_at = None;
                let mut payload = None;
                let mut metadata = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "event_id" => event_id = Some(map.next_value()?),
                        "event_name" => event_name = Some(map.next_value()?),
                        "event_version" => event_version = Some(map.next_value()?),
                        "aggregate_id" => aggregate_id = Some(map.next_value()?),
                        "aggregate_type" => aggregate_type = Some(map.next_value()?),
                        "aggregate_version" => aggregate_version = Some(map.next_value()?),
                        "tenant_id" => tenant_id = Some(map.next_value()?),
                        "occurred_at" => occurred_at = Some(map.next_value()?),
                        "payload" => payload = Some(map.next_value()?),
                        "metadata" => metadata = Some(map.next_value()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                Ok(EventEnvelope {
                    event_id: event_id.unwrap_or_default(),
                    event_name: event_name.unwrap_or_default(),
                    event_version: event_version.unwrap_or(1),
                    aggregate_id: aggregate_id.unwrap_or_default(),
                    aggregate_type: aggregate_type.unwrap_or_default(),
                    aggregate_version: aggregate_version.unwrap_or(0),
                    tenant_id: tenant_id.unwrap_or_default(),
                    occurred_at: occurred_at.unwrap_or_else(Utc::now),
                    payload: payload.unwrap_or_else(|| panic!("payload is required")),
                    metadata: metadata.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_map(EventEnvelopeVisitor {
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<E: DomainEvent> EventEnvelope<E> {
    /// 创建新的事件信封
    pub fn new(
        aggregate_id: impl Into<String>,
        aggregate_type: impl Into<String>,
        aggregate_version: i64,
        tenant_id: impl Into<String>,
        payload: E,
    ) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            event_name: E::event_name().to_string(),
            event_version: E::event_version(),
            aggregate_id: aggregate_id.into(),
            aggregate_type: aggregate_type.into(),
            aggregate_version,
            tenant_id: tenant_id.into(),
            occurred_at: Utc::now(),
            payload,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// 序列化为 JSON
    pub fn to_json(&self) -> Result<String, AppError> {
        serde_json::to_string(self)
            .map_err(|e| AppError::Internal(format!("Failed to serialize event: {}", e)))
    }

    /// 从 JSON 反序列化
    pub fn from_json(json: &str) -> Result<Self, AppError>
    where
        E: DeserializeOwned,
    {
        serde_json::from_str(json)
            .map_err(|e| AppError::Internal(format!("Failed to deserialize event: {}", e)))
    }
}

/// Outbox 事件记录
///
/// 用于持久化到 Outbox 表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxEvent {
    /// 事件 ID
    pub event_id: Uuid,

    /// 事件名称
    pub event_name: String,

    /// 聚合根 ID
    pub aggregate_id: String,

    /// 聚合根类型
    pub aggregate_type: String,

    /// 租户 ID
    pub tenant_id: String,

    /// 事件负载（JSON）
    pub payload: String,

    /// 发生时间
    pub occurred_at: DateTime<Utc>,

    /// 是否已发布
    pub published: bool,

    /// 发布时间
    pub published_at: Option<DateTime<Utc>>,

    /// 重试次数
    pub retry_count: i32,

    /// 最后错误
    pub last_error: Option<String>,
}

impl OutboxEvent {
    /// 从事件信封创建
    pub fn from_envelope<E: DomainEvent>(envelope: &EventEnvelope<E>) -> Result<Self, AppError> {
        Ok(Self {
            event_id: envelope.event_id,
            event_name: envelope.event_name.clone(),
            aggregate_id: envelope.aggregate_id.clone(),
            aggregate_type: envelope.aggregate_type.clone(),
            tenant_id: envelope.tenant_id.clone(),
            payload: envelope.to_json()?,
            occurred_at: envelope.occurred_at,
            published: false,
            published_at: None,
            retry_count: 0,
            last_error: None,
        })
    }
}

/// Outbox 仓储接口
///
/// 用于持久化事件到 Outbox 表
#[async_trait]
pub trait OutboxRepository: Send + Sync {
    /// 保存事件到 Outbox
    async fn save(&self, event: OutboxEvent) -> Result<(), AppError>;

    /// 批量保存事件
    async fn save_batch(&self, events: Vec<OutboxEvent>) -> Result<(), AppError>;

    /// 获取未发布的事件
    async fn get_unpublished(&self, limit: usize) -> Result<Vec<OutboxEvent>, AppError>;

    /// 标记事件为已发布
    async fn mark_published(&self, event_id: Uuid) -> Result<(), AppError>;

    /// 标记事件为失败
    async fn mark_failed(&self, event_id: Uuid, error: String) -> Result<(), AppError>;

    /// 删除已发布的旧事件
    async fn delete_published_before(&self, before: DateTime<Utc>) -> Result<u64, AppError>;
}

/// 事件处理器
///
/// 处理特定类型的领域事件
#[async_trait]
pub trait EventHandler<E: DomainEvent>: Send + Sync {
    /// 处理事件
    async fn handle(&self, envelope: &EventEnvelope<E>) -> Result<(), AppError>;
}

/// 事件总线
///
/// 负责分发事件到处理器
#[async_trait]
pub trait EventBus: Send + Sync {
    /// 发布事件（同步/内存）
    ///
    /// 直接在内存中触发事件处理器
    async fn publish_sync<E: DomainEvent>(&self, envelope: EventEnvelope<E>) -> Result<(), AppError>;

    /// 发布事件（异步/Outbox）
    ///
    /// 将事件保存到 Outbox 表，由后台 worker 异步发布
    async fn publish_async<E: DomainEvent>(&self, envelope: EventEnvelope<E>) -> Result<(), AppError>;

    /// 批量发布事件（异步/Outbox）
    async fn publish_async_batch<E: DomainEvent>(
        &self,
        envelopes: Vec<EventEnvelope<E>>,
    ) -> Result<(), AppError>;
}
