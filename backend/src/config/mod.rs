use app::AppConfig;
use database::DatabaseConfig;
use zirv_config::register_config;

mod app;
mod database;

pub fn register_configs() {
    register_config!("app", AppConfig::default());
    register_config!("database", DatabaseConfig::default());
}
