# Kafka Module Usage Examples

This document provides comprehensive examples of how to use the Kafka module in your application.

## Overview

The Kafka module provides an easy-to-configure system for working with Kafka consumers and producers. It includes:

- **KafkaConfig**: Configuration management via environment variables
- **MessageHandler trait**: Define custom message processing logic for each topic
- **KafkaConsumer**: Automatically consume messages from subscribed topics
- **KafkaProducer**: Send messages to Kafka topics
- **KafkaManager**: Orchestrate consumers and producers

## Configuration

Configure Kafka using environment variables:

```bash
# Enable Kafka functionality
export KAFKA_ENABLED=true

# Kafka broker addresses (comma-separated)
export KAFKA_BROKERS=localhost:9092

# Consumer group ID
export KAFKA_CONSUMER_GROUP_ID=template-service-group

# Topics to consume from (comma-separated, but better to register handlers directly)
export KAFKA_TOPICS=user-events,notifications

# Auto-offset reset strategy (earliest, latest, none)
export KAFKA_AUTO_OFFSET_RESET=latest

# Session timeout in milliseconds
export KAFKA_SESSION_TIMEOUT_MS=6000
```

## Basic Usage

### 1. Create a Message Handler

Implement the `MessageHandler` trait for each topic you want to consume from:

```rust
use async_trait::async_trait;
use backend::kafka::{MessageHandler, MessageAction, HandlerResult};

pub struct MyTopicHandler {
    topic: String,
}

impl MyTopicHandler {
    pub fn new() -> Self {
        Self {
            topic: "my-topic".to_string(),
        }
    }
}

#[async_trait]
impl MessageHandler for MyTopicHandler {
    fn topic(&self) -> &str {
        &self.topic
    }

    async fn handle(
        &self,
        key: Option<&[u8]>,
        payload: &[u8],
        topic: &str,
        partition: i32,
        offset: i64,
    ) -> HandlerResult {
        // Your business logic here
        tracing::info!(
            topic = %topic,
            partition = partition,
            offset = offset,
            "Processing message"
        );

        // Parse the payload
        let message = String::from_utf8_lossy(payload);
        
        // Decide what to do with the message
        if message.is_empty() {
            // Skip empty messages (will not commit, will retry)
            Ok(MessageAction::Skip)
        } else if message.contains("error") {
            // Reject invalid messages (will commit, will not retry)
            Ok(MessageAction::Reject)
        } else {
            // Process and consume the message
            // ... your business logic ...
            Ok(MessageAction::Consume)
        }
    }
}
```

### 2. Initialize Kafka Manager

In your `main.rs` or application initialization:

```rust
use std::sync::Arc;
use backend::config::KafkaConfig;
use backend::kafka::KafkaManager;
use zirv_config::read_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... other initialization ...

    // Read Kafka configuration
    let kafka_config = read_config!("kafka", KafkaConfig).unwrap();

    // Create Kafka manager
    let mut kafka_manager = KafkaManager::new(kafka_config)
        .expect("Failed to create Kafka manager");

    // Register message handlers for each topic
    kafka_manager
        .register_handler(Arc::new(MyTopicHandler::new()))
        .register_handler(Arc::new(AnotherTopicHandler::new()));

    // Start consuming messages in the background
    if let Some(consumer_task) = kafka_manager.start_consumer() {
        // The consumer runs in the background
        // You can store the handle if you need to manage it
        tokio::spawn(async move {
            consumer_task.await.ok();
        });
    }

    // Get producer for sending messages
    if let Some(producer) = kafka_manager.producer() {
        // Send a message
        producer.send("my-topic", Some("key"), b"Hello, Kafka!")
            .await
            .expect("Failed to send message");

        // Send JSON data
        use serde::Serialize;
        
        #[derive(Serialize)]
        struct MyData {
            id: String,
            value: i32,
        }
        
        let data = MyData {
            id: "123".to_string(),
            value: 42,
        };
        
        producer.send_json("my-topic", Some("key"), &data)
            .await
            .expect("Failed to send JSON message");
    }

    // ... start your HTTP server ...
    
    Ok(())
}
```

## Advanced Examples

### JSON Message Handler

Process JSON messages with validation:

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use backend::kafka::{MessageHandler, MessageAction, HandlerResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserEvent {
    pub user_id: String,
    pub event_type: String,
    pub timestamp: i64,
}

pub struct UserEventHandler {
    topic: String,
}

impl UserEventHandler {
    pub fn new() -> Self {
        Self {
            topic: "user-events".to_string(),
        }
    }
}

#[async_trait]
impl MessageHandler for UserEventHandler {
    fn topic(&self) -> &str {
        &self.topic
    }

    async fn handle(
        &self,
        _key: Option<&[u8]>,
        payload: &[u8],
        topic: &str,
        partition: i32,
        offset: i64,
    ) -> HandlerResult {
        // Parse JSON payload
        match serde_json::from_slice::<UserEvent>(payload) {
            Ok(event) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    user_id = %event.user_id,
                    event_type = %event.event_type,
                    "Processing user event"
                );

                // Validate event
                let valid_types = ["login", "logout", "signup", "purchase"];
                if !valid_types.contains(&event.event_type.as_str()) {
                    tracing::warn!(event_type = %event.event_type, "Invalid event type");
                    return Ok(MessageAction::Reject);
                }

                // Process the event
                // ... your business logic ...

                Ok(MessageAction::Consume)
            }
            Err(e) => {
                tracing::error!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    error = ?e,
                    "Failed to parse JSON"
                );
                Ok(MessageAction::Reject)
            }
        }
    }
}
```

### Message Transformation

Transform messages before processing:

```rust
use async_trait::async_trait;
use backend::kafka::{MessageHandler, MessageAction, HandlerResult};

pub struct TransformingHandler {
    topic: String,
}

#[async_trait]
impl MessageHandler for TransformingHandler {
    fn topic(&self) -> &str {
        &self.topic
    }

    fn transform(
        &self,
        payload: &[u8]
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Example: Convert to uppercase
        let text = String::from_utf8_lossy(payload);
        let transformed = text.to_uppercase();
        Ok(transformed.into_bytes())
    }

    async fn handle(
        &self,
        _key: Option<&[u8]>,
        payload: &[u8],
        _topic: &str,
        _partition: i32,
        _offset: i64,
    ) -> HandlerResult {
        // The payload here is already transformed
        let message = String::from_utf8_lossy(payload);
        tracing::info!(transformed_message = %message, "Processing transformed message");
        
        Ok(MessageAction::Consume)
    }
}
```

### Error Handling

Handle errors gracefully:

```rust
use async_trait::async_trait;
use backend::kafka::{MessageHandler, MessageAction, HandlerResult};

pub struct RobustHandler {
    topic: String,
}

#[async_trait]
impl MessageHandler for RobustHandler {
    fn topic(&self) -> &str {
        &self.topic
    }

    async fn handle(
        &self,
        _key: Option<&[u8]>,
        payload: &[u8],
        topic: &str,
        partition: i32,
        offset: i64,
    ) -> HandlerResult {
        // Try to process the message
        match self.process_message(payload).await {
            Ok(_) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    "Message processed successfully"
                );
                Ok(MessageAction::Consume)
            }
            Err(e) if e.is_retryable() => {
                // Temporary error, skip and retry later
                tracing::warn!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    error = ?e,
                    "Temporary error, will retry"
                );
                Ok(MessageAction::Skip)
            }
            Err(e) => {
                // Permanent error, reject the message
                tracing::error!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    error = ?e,
                    "Permanent error, rejecting message"
                );
                Ok(MessageAction::Reject)
            }
        }
    }
}

impl RobustHandler {
    async fn process_message(
        &self,
        _payload: &[u8]
    ) -> Result<(), ProcessingError> {
        // Your processing logic here
        Ok(())
    }
}

#[derive(Debug)]
enum ProcessingError {
    Network(String),
    InvalidData(String),
}

impl ProcessingError {
    fn is_retryable(&self) -> bool {
        matches!(self, ProcessingError::Network(_))
    }
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::Network(msg) => write!(f, "Network error: {}", msg),
            ProcessingError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for ProcessingError {}
```

## Message Actions

The handler returns a `MessageAction` to indicate how to proceed:

- **`MessageAction::Consume`**: Successfully processed the message. The offset will be committed, and the message won't be reprocessed.

- **`MessageAction::Skip`**: The message couldn't be processed right now but might succeed later (e.g., temporary service unavailability). The offset won't be committed, and the message will be retried on the next poll.

- **`MessageAction::Reject`**: The message is invalid and should never be retried (e.g., malformed JSON, invalid schema). The offset will be committed to prevent reprocessing.

## Testing

Test your handlers using the provided examples:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler() {
        let handler = MyTopicHandler::new();
        
        let payload = b"test message";
        let result = handler
            .handle(None, payload, "my-topic", 0, 1)
            .await
            .unwrap();
        
        assert_eq!(result, MessageAction::Consume);
    }
}
```

## Running with Docker Compose

For local development, you can use Docker Compose to run Kafka:

```yaml
version: '3.8'
services:
  zookeeper:
    image: confluentinc/cp-zookeeper:latest
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000

  kafka:
    image: confluentinc/cp-kafka:latest
    depends_on:
      - zookeeper
    ports:
      - "9092:9092"
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
```

Then run:

```bash
docker-compose up -d
```

## Troubleshooting

### Consumer Not Processing Messages

- Check that `KAFKA_ENABLED=true` is set
- Verify broker address is correct
- Ensure topics exist in Kafka
- Check that handlers are registered before calling `start_consumer()`
- Review logs for connection errors

### Messages Not Being Committed

- Verify your handler returns `MessageAction::Consume` or `MessageAction::Reject`
- Check for errors in the logs
- Ensure the consumer has proper permissions

### Performance Issues

- Adjust `session_timeout_ms` based on your processing time
- Consider using multiple consumer groups for parallel processing
- Monitor Kafka consumer lag
