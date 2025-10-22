use std::sync::Arc;
use tokio::task::JoinHandle;

use super::config::KafkaConfig;
use super::consumer::KafkaConsumer;
use super::handler::MessageHandler;
use super::producer::KafkaProducer;

/// Manager for Kafka consumers and producers
///
/// This is the main entry point for working with Kafka in your application.
/// It handles:
/// - Creating and configuring consumers and producers
/// - Registering message handlers for topics
/// - Starting consumer tasks
pub struct KafkaManager {
    config: KafkaConfig,
    consumer: Option<KafkaConsumer>,
    producer: Option<KafkaProducer>,
}

impl KafkaManager {
    /// Create a new KafkaManager from configuration
    pub fn new(config: KafkaConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !config.enabled {
            tracing::info!("Kafka is disabled, skipping initialization");
            return Ok(Self { config, consumer: None, producer: None });
        }

        tracing::info!(brokers = %config.brokers, "Initializing Kafka manager");

        let consumer = Some(KafkaConsumer::new(&config)?);
        let producer = Some(KafkaProducer::new(&config)?);

        Ok(Self { config, consumer, producer })
    }

    /// Register a message handler for a topic
    ///
    /// The handler will be called for each message received from the topic.
    /// Multiple handlers can be registered for different topics.
    pub fn register_handler(&mut self, handler: Arc<dyn MessageHandler>) -> &mut Self {
        if let Some(consumer) = &mut self.consumer {
            consumer.register_handler(handler);
        } else {
            tracing::warn!("Kafka is disabled, cannot register handler");
        }
        self
    }

    /// Start consuming messages from all registered topics
    ///
    /// This will spawn a background task that continuously processes messages.
    /// Returns a JoinHandle that can be used to manage the consumer task.
    pub fn start_consumer(&mut self) -> Option<JoinHandle<()>> {
        if let Some(consumer) = self.consumer.take() {
            match consumer.subscribe() {
                | Ok(_) => {
                    tracing::info!("Starting Kafka consumer");
                    Some(consumer.start())
                }
                | Err(e) => {
                    tracing::error!(error = ?e, "Failed to subscribe to topics");
                    None
                }
            }
        } else {
            tracing::warn!("Kafka consumer not available");
            None
        }
    }

    /// Get a reference to the producer
    pub fn producer(&self) -> Option<&KafkaProducer> {
        self.producer.as_ref()
    }

    /// Check if Kafka is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::super::handler::{HandlerResult, MessageAction, MessageHandler};
    use super::*;
    use async_trait::async_trait;

    struct TestHandler {
        topic: String,
    }

    #[async_trait]
    impl MessageHandler for TestHandler {
        fn topic(&self) -> &str {
            &self.topic
        }

        async fn handle(
            &self,
            _key: Option<&[u8]>,
            _payload: &[u8],
            _topic: &str,
            _partition: i32,
            _offset: i64,
        ) -> HandlerResult {
            Ok(MessageAction::Consume)
        }
    }

    #[tokio::test]
    async fn test_manager_creation_disabled() {
        let config = KafkaConfig {
            brokers: "localhost:9092".to_string(),
            consumer_group_id: "test-group".to_string(),
            topics: "test-topic".to_string(),
            enabled: false,
            auto_offset_reset: "latest".to_string(),
            session_timeout_ms: 6000,
        };

        let manager = KafkaManager::new(config).unwrap();
        assert!(!manager.is_enabled());
        assert!(manager.consumer.is_none());
        assert!(manager.producer.is_none());
    }

    #[tokio::test]
    async fn test_manager_creation_enabled() {
        let config = KafkaConfig {
            brokers: "localhost:9092".to_string(),
            consumer_group_id: "test-group".to_string(),
            topics: "test-topic".to_string(),
            enabled: true,
            auto_offset_reset: "latest".to_string(),
            session_timeout_ms: 6000,
        };

        let manager = KafkaManager::new(config).unwrap();
        assert!(manager.is_enabled());
        assert!(manager.consumer.is_some());
        assert!(manager.producer.is_some());
    }

    #[tokio::test]
    async fn test_register_handler() {
        let config = KafkaConfig {
            brokers: "localhost:9092".to_string(),
            consumer_group_id: "test-group".to_string(),
            topics: "test-topic".to_string(),
            enabled: true,
            auto_offset_reset: "latest".to_string(),
            session_timeout_ms: 6000,
        };

        let mut manager = KafkaManager::new(config).unwrap();
        let handler = Arc::new(TestHandler { topic: "test-topic".to_string() });

        manager.register_handler(handler);
        // The handler is registered inside the consumer, so we can't directly check it here
        // but we can verify that the method doesn't panic
    }

    #[tokio::test]
    async fn test_register_handler_when_disabled() {
        let config = KafkaConfig {
            brokers: "localhost:9092".to_string(),
            consumer_group_id: "test-group".to_string(),
            topics: "test-topic".to_string(),
            enabled: false,
            auto_offset_reset: "latest".to_string(),
            session_timeout_ms: 6000,
        };

        let mut manager = KafkaManager::new(config).unwrap();
        let handler = Arc::new(TestHandler { topic: "test-topic".to_string() });

        // Should not panic even when disabled
        manager.register_handler(handler);
    }
}
