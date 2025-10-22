use app::AppConfig;
use database::DatabaseConfig;
use zirv_config::register_config;

pub use crate::kafka::KafkaConfig;
pub use logging::LoggingConfig;

mod app;
mod database;
pub mod logging;

pub fn register_configs() {
    register_config!("app", AppConfig::default());
    register_config!("database", DatabaseConfig::default());
    register_config!("logging", LoggingConfig::default());
    register_config!("kafka", KafkaConfig::default());
}
