use serde::{Deserialize, Serialize};

use common::utils::env_or_default;

#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig {
    /// Interval in milliseconds at which some recurring task or loop should tick.
    /// Defaults to `1000` if not set.
    #[serde(default)]
    pub host: String,

    /// Port on which the server should listen.
    /// Defaults to `3000` if not set.
    #[serde(default)]
    pub port: i32,

    /// Application environment (e.g., "development", "production").
    /// Defaults to "development" if not set.
    #[serde(default)]
    pub environment: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: env_or_default("HOST", "0.0.0.0".to_string()),
            port: env_or_default("PORT", 3000),
            environment: env_or_default("ENVIRONMENT", "development".to_string()),
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
            std::env::remove_var("HOST");
        }
        unsafe {
            std::env::remove_var("PORT");
        }
        unsafe {
            std::env::remove_var("ENVIRONMENT");
        }
        let cfg = AppConfig::default();
        assert_eq!(cfg.host, "0.0.0.0");
        assert_eq!(cfg.port, 3000);
        assert_eq!(cfg.environment, "development");
    }

    #[test]
    #[serial]
    fn test_env_overrides() {
        unsafe {
            std::env::set_var("HOST", "127.0.0.1");
        }
        unsafe {
            std::env::set_var("PORT", "4321");
        }
        unsafe {
            std::env::set_var("ENVIRONMENT", "prod");
        }
        let cfg = AppConfig::default();
        assert_eq!(cfg.host, "127.0.0.1");
        assert_eq!(cfg.port, 4321);
        assert_eq!(cfg.environment, "prod");
        unsafe {
            std::env::remove_var("HOST");
        }
        unsafe {
            std::env::remove_var("PORT");
        }
        unsafe {
            std::env::remove_var("ENVIRONMENT");
        }
    }
}
