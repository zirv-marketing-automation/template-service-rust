use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;

use super::config::KafkaConfig;
use super::handler::{MessageAction, MessageHandler};

/// Kafka consumer for processing messages from topics
pub struct KafkaConsumer {
    consumer: StreamConsumer,
    handlers: HashMap<String, Arc<dyn MessageHandler>>,
}

impl KafkaConsumer {
    /// Create a new Kafka consumer from configuration
    pub fn new(config: &KafkaConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", &config.consumer_group_id)
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", &config.auto_offset_reset)
            .set("session.timeout.ms", config.session_timeout_ms.to_string())
            .create()?;

        Ok(Self { consumer, handlers: HashMap::new() })
    }

    /// Register a message handler for a specific topic
    pub fn register_handler(&mut self, handler: Arc<dyn MessageHandler>) {
        let topic = handler.topic().to_string();
        tracing::info!(topic = %topic, "Registering message handler");
        self.handlers.insert(topic, handler);
    }

    /// Subscribe to topics that have registered handlers
    pub fn subscribe(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let topics: Vec<&str> = self.handlers.keys().map(|s| s.as_str()).collect();

        if topics.is_empty() {
            tracing::warn!("No handlers registered, skipping subscription");
            return Ok(());
        }

        tracing::info!(topics = ?topics, "Subscribing to topics");
        self.consumer.subscribe(&topics)?;
        Ok(())
    }

    /// Start consuming messages in a background task
    ///
    /// Returns a JoinHandle that can be used to manage the consumer task
    pub fn start(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            self.consume_loop().await;
        })
    }

    /// Main consumption loop
    async fn consume_loop(self) {
        tracing::info!("Starting Kafka consumer loop");

        loop {
            match self.consumer.recv().await {
                | Ok(message) => {
                    self.process_message(message).await;
                }
                | Err(e) => {
                    tracing::error!(error = ?e, "Error receiving message from Kafka");
                }
            }
        }
    }

    /// Process a single message using the appropriate handler
    async fn process_message(&self, message: rdkafka::message::BorrowedMessage<'_>) {
        let topic = message.topic();
        let partition = message.partition();
        let offset = message.offset();

        tracing::debug!(
            topic = %topic,
            partition = partition,
            offset = offset,
            "Processing message"
        );

        // Find the handler for this topic
        let handler = match self.handlers.get(topic) {
            | Some(h) => h,
            | None => {
                tracing::warn!(
                    topic = %topic,
                    "No handler registered for topic, skipping message"
                );
                return;
            }
        };

        // Get message payload
        let payload = match message.payload() {
            | Some(p) => p,
            | None => {
                tracing::warn!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    "Message has no payload, skipping"
                );
                return;
            }
        };

        // Get message key if present
        let key = message.key();

        // Call the handler
        match handler.handle(key, payload, topic, partition, offset).await {
            | Ok(MessageAction::Consume) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    "Message consumed successfully"
                );
                if let Err(e) = self
                    .consumer
                    .commit_message(&message, rdkafka::consumer::CommitMode::Async)
                {
                    tracing::error!(
                        topic = %topic,
                        partition = partition,
                        offset = offset,
                        error = ?e,
                        "Failed to commit message"
                    );
                }
            }
            | Ok(MessageAction::Skip) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    "Message skipped by handler"
                );
            }
            | Ok(MessageAction::Reject) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    "Message rejected by handler"
                );
                if let Err(e) = self
                    .consumer
                    .commit_message(&message, rdkafka::consumer::CommitMode::Async)
                {
                    tracing::error!(
                        topic = %topic,
                        partition = partition,
                        offset = offset,
                        error = ?e,
                        "Failed to commit rejected message"
                    );
                }
            }
            | Err(e) => {
                tracing::error!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    error = ?e,
                    "Handler failed to process message"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::handler::{HandlerResult, MessageHandler};
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
    async fn test_consumer_creation() {
        let config = KafkaConfig {
            brokers: "localhost:9092".to_string(),
            consumer_group_id: "test-group".to_string(),
            topics: "test-topic".to_string(),
            enabled: true,
            auto_offset_reset: "latest".to_string(),
            session_timeout_ms: 6000,
        };

        let result = KafkaConsumer::new(&config);
        assert!(result.is_ok());
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

        let mut consumer = KafkaConsumer::new(&config).unwrap();
        let handler = Arc::new(TestHandler { topic: "test-topic".to_string() });

        consumer.register_handler(handler);
        assert_eq!(consumer.handlers.len(), 1);
        assert!(consumer.handlers.contains_key("test-topic"));
    }
}
