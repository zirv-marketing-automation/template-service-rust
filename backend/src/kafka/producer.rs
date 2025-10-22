use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

use super::config::KafkaConfig;

/// Kafka producer for sending messages to topics
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    /// Create a new Kafka producer from configuration
    pub fn new(config: &KafkaConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer })
    }

    /// Send a message to a Kafka topic
    ///
    /// # Arguments
    /// * `topic` - The topic to send the message to
    /// * `key` - Optional message key
    /// * `payload` - The message payload
    ///
    /// # Returns
    /// The partition and offset where the message was written
    pub async fn send(
        &self,
        topic: &str,
        key: Option<&str>,
        payload: &[u8],
    ) -> Result<(i32, i64), Box<dyn std::error::Error + Send + Sync>> {
        let mut record = FutureRecord::to(topic).payload(payload);

        if let Some(k) = key {
            record = record.key(k);
        }

        let delivery_status = self
            .producer
            .send(record, Timeout::After(Duration::from_secs(5)))
            .await;

        match delivery_status {
            | Ok((partition, offset)) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    "Message sent successfully"
                );
                Ok((partition, offset))
            }
            | Err((e, _)) => {
                tracing::error!(
                    topic = %topic,
                    error = ?e,
                    "Failed to send message"
                );
                Err(Box::new(e))
            }
        }
    }

    /// Send a JSON-serializable message to a Kafka topic
    ///
    /// # Arguments
    /// * `topic` - The topic to send the message to
    /// * `key` - Optional message key
    /// * `data` - The data to serialize and send
    pub async fn send_json<T: serde::Serialize>(
        &self,
        topic: &str,
        key: Option<&str>,
        data: &T,
    ) -> Result<(i32, i64), Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::to_vec(data)?;
        self.send(topic, key, &payload).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_producer_creation() {
        let config = KafkaConfig {
            brokers: "localhost:9092".to_string(),
            consumer_group_id: "test-group".to_string(),
            topics: "test-topic".to_string(),
            enabled: true,
            auto_offset_reset: "latest".to_string(),
            session_timeout_ms: 6000,
        };

        // Note: This will fail if Kafka is not running, but it tests that we can create the producer
        let result = KafkaProducer::new(&config);
        assert!(result.is_ok());
    }
}
