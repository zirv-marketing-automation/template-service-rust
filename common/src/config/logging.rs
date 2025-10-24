use serde::{Deserialize, Serialize};

use crate::utils::env_or_default;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoggingConfig {
    /// Log level (e.g., "trace", "debug", "info", "warn", "error")
    /// Defaults to "info" if not set.
    #[serde(default)]
    pub level: String,

    /// Log format: "json" for structured logs (Kibana), "pretty" for human-readable
    /// Defaults to "json" for production use with Kibana.
    #[serde(default)]
    pub format: String,

    /// Service name to include in logs for identification in Kibana
    /// Defaults to "template-service" if not set.
    #[serde(default)]
    pub service_name: String,

    /// Environment name to include in logs (e.g., "production", "staging", "development")
    /// Defaults to "production" if not set.
    #[serde(default)]
    pub environment: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: env_or_default("LOG_LEVEL", "info".to_string()),
            format: env_or_default("LOG_FORMAT", "json".to_string()),
            service_name: env_or_default("SERVICE_NAME", "template-service".to_string()),
            environment: env_or_default("ENVIRONMENT", "production".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_default_values() {
        unsafe {
            std::env::remove_var("LOG_LEVEL");
            std::env::remove_var("LOG_FORMAT");
            std::env::remove_var("SERVICE_NAME");
            std::env::remove_var("ENVIRONMENT");
        }
        let cfg = LoggingConfig::default();
        assert_eq!(cfg.level, "info");
        assert_eq!(cfg.format, "json");
        assert_eq!(cfg.service_name, "template-service");
        assert_eq!(cfg.environment, "production");
    }

    #[test]
    #[serial]
    fn test_env_overrides() {
        unsafe {
            std::env::set_var("LOG_LEVEL", "debug");
            std::env::set_var("LOG_FORMAT", "pretty");
            std::env::set_var("SERVICE_NAME", "test-service");
            std::env::set_var("ENVIRONMENT", "development");
        }
        let cfg = LoggingConfig::default();
        assert_eq!(cfg.level, "debug");
        assert_eq!(cfg.format, "pretty");
        assert_eq!(cfg.service_name, "test-service");
        assert_eq!(cfg.environment, "development");
        unsafe {
            std::env::remove_var("LOG_LEVEL");
            std::env::remove_var("LOG_FORMAT");
            std::env::remove_var("SERVICE_NAME");
            std::env::remove_var("ENVIRONMENT");
        }
    }
}
