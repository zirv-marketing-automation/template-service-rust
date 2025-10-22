use serde::{Deserialize, Serialize};

use crate::utils::env_or_default;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KafkaConfig {
    /// Comma-separated list of Kafka broker addresses (e.g., "localhost:9092")
    #[serde(default)]
    pub brokers: String,

    /// Consumer group ID for Kafka consumers
    #[serde(default)]
    pub consumer_group_id: String,

    /// Comma-separated list of topics to consume from
    #[serde(default)]
    pub topics: String,

    /// Whether to enable Kafka functionality
    #[serde(default)]
    pub enabled: bool,

    /// Auto-offset reset strategy for consumers (earliest, latest, none)
    #[serde(default)]
    pub auto_offset_reset: String,

    /// Session timeout in milliseconds
    #[serde(default)]
    pub session_timeout_ms: u32,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: env_or_default("KAFKA_BROKERS", "localhost:9092".to_string()),
            consumer_group_id: env_or_default(
                "KAFKA_CONSUMER_GROUP_ID",
                "template-service-group".to_string(),
            ),
            topics: env_or_default("KAFKA_TOPICS", "".to_string()),
            enabled: env_or_default("KAFKA_ENABLED", false),
            auto_offset_reset: env_or_default("KAFKA_AUTO_OFFSET_RESET", "latest".to_string()),
            session_timeout_ms: env_or_default("KAFKA_SESSION_TIMEOUT_MS", 6000),
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
            std::env::remove_var("KAFKA_BROKERS");
        }
        unsafe {
            std::env::remove_var("KAFKA_CONSUMER_GROUP_ID");
        }
        unsafe {
            std::env::remove_var("KAFKA_TOPICS");
        }
        unsafe {
            std::env::remove_var("KAFKA_ENABLED");
        }
        unsafe {
            std::env::remove_var("KAFKA_AUTO_OFFSET_RESET");
        }
        unsafe {
            std::env::remove_var("KAFKA_SESSION_TIMEOUT_MS");
        }
        let cfg = KafkaConfig::default();
        assert_eq!(cfg.brokers, "localhost:9092");
        assert_eq!(cfg.consumer_group_id, "template-service-group");
        assert_eq!(cfg.topics, "");
        assert!(!cfg.enabled);
        assert_eq!(cfg.auto_offset_reset, "latest");
        assert_eq!(cfg.session_timeout_ms, 6000);
    }

    #[test]
    #[serial]
    fn test_env_overrides() {
        unsafe {
            std::env::set_var("KAFKA_BROKERS", "broker1:9092,broker2:9092");
        }
        unsafe {
            std::env::set_var("KAFKA_CONSUMER_GROUP_ID", "my-group");
        }
        unsafe {
            std::env::set_var("KAFKA_TOPICS", "topic1,topic2");
        }
        unsafe {
            std::env::set_var("KAFKA_ENABLED", "true");
        }
        unsafe {
            std::env::set_var("KAFKA_AUTO_OFFSET_RESET", "earliest");
        }
        unsafe {
            std::env::set_var("KAFKA_SESSION_TIMEOUT_MS", "10000");
        }
        let cfg = KafkaConfig::default();
        assert_eq!(cfg.brokers, "broker1:9092,broker2:9092");
        assert_eq!(cfg.consumer_group_id, "my-group");
        assert_eq!(cfg.topics, "topic1,topic2");
        assert!(cfg.enabled);
        assert_eq!(cfg.auto_offset_reset, "earliest");
        assert_eq!(cfg.session_timeout_ms, 10000);
        unsafe {
            std::env::remove_var("KAFKA_BROKERS");
        }
        unsafe {
            std::env::remove_var("KAFKA_CONSUMER_GROUP_ID");
        }
        unsafe {
            std::env::remove_var("KAFKA_TOPICS");
        }
        unsafe {
            std::env::remove_var("KAFKA_ENABLED");
        }
        unsafe {
            std::env::remove_var("KAFKA_AUTO_OFFSET_RESET");
        }
        unsafe {
            std::env::remove_var("KAFKA_SESSION_TIMEOUT_MS");
        }
    }
}
