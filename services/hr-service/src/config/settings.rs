use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub kafka_brokers: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database_url: format!(
                "postgres://{}:password@localhost:5432/{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_NAME")
            ),
            kafka_brokers: "localhost:9092".into(),
        }
    }
}
