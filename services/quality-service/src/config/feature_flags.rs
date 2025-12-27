#[derive(Debug, Default, Clone)]
pub struct FeatureFlags {
    pub enable_projection_rebuilds: bool,
    pub enable_async_validation: bool,
}
