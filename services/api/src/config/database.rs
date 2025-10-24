use serde::{Deserialize, Serialize};

use common::utils::env_or_default;

#[derive(Deserialize, Debug, Serialize)]
pub struct DatabaseConfig {
    /// The URL for your database connection. Must be set via `DATABASE_URL`.
    pub url: String,

    /// Maximum number of database connetions the application will handle.
    /// Defaults to `5` if not present in the environment.
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: env_or_default("DATABASE_URL", "0.0.0.0".to_string()),
            max_connections: env_or_default("MAX_DATABASE_CONNECTIONS", 5),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        unsafe {
            std::env::remove_var("DATABASE_URL");
        }
        unsafe {
            std::env::remove_var("MAX_DATABASE_CONNECTIONS");
        }
        let cfg = DatabaseConfig::default();
        assert_eq!(cfg.url, "0.0.0.0");
        assert_eq!(cfg.max_connections, 5);
    }
}
