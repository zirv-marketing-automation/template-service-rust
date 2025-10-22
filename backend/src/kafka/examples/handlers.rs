use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::super::handler::{HandlerResult, MessageAction, MessageHandler};

/// Example data structure for user events
#[derive(Debug, Deserialize, Serialize)]
pub struct UserEvent {
    pub user_id: String,
    pub event_type: String,
    pub timestamp: i64,
}

/// Example handler for processing user events from a Kafka topic
pub struct UserEventHandler {
    topic: String,
}

impl UserEventHandler {
    pub fn new(topic: &str) -> Self {
        Self { topic: topic.to_string() }
    }
}

#[async_trait]
impl MessageHandler for UserEventHandler {
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
        // Log the message key if present
        if let Some(k) = key {
            tracing::debug!(
                key = ?String::from_utf8_lossy(k),
                "Processing message with key"
            );
        }

        // Try to parse the payload as JSON
        match serde_json::from_slice::<UserEvent>(payload) {
            | Ok(event) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    user_id = %event.user_id,
                    event_type = %event.event_type,
                    "Processing user event"
                );

                // Example: Skip events that are too old (more than 30 days)
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;

                if now - event.timestamp > 30 * 24 * 60 * 60 {
                    tracing::warn!(
                        user_id = %event.user_id,
                        age_days = (now - event.timestamp) / (24 * 60 * 60),
                        "Event is too old, skipping"
                    );
                    return Ok(MessageAction::Skip);
                }

                // Example: Reject events with invalid event types
                let valid_types = ["login", "logout", "signup", "purchase"];
                if !valid_types.contains(&event.event_type.as_str()) {
                    tracing::warn!(
                        event_type = %event.event_type,
                        "Invalid event type, rejecting"
                    );
                    return Ok(MessageAction::Reject);
                }

                // Process the event (your business logic here)
                // ...

                Ok(MessageAction::Consume)
            }
            | Err(e) => {
                tracing::error!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    error = ?e,
                    "Failed to parse message as UserEvent"
                );
                // Reject malformed messages
                Ok(MessageAction::Reject)
            }
        }
    }

    fn transform(
        &self,
        payload: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        // Example: You could transform the payload here if needed
        // For now, just return it as-is
        Ok(payload.to_vec())
    }
}

/// Example handler for processing simple string messages
pub struct SimpleStringHandler {
    topic: String,
}

impl SimpleStringHandler {
    pub fn new(topic: &str) -> Self {
        Self { topic: topic.to_string() }
    }
}

#[async_trait]
impl MessageHandler for SimpleStringHandler {
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
        match String::from_utf8(payload.to_vec()) {
            | Ok(message) => {
                tracing::info!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    message = %message,
                    "Processing string message"
                );

                // Skip empty messages
                if message.trim().is_empty() {
                    return Ok(MessageAction::Skip);
                }

                // Process the message
                // ...

                Ok(MessageAction::Consume)
            }
            | Err(e) => {
                tracing::error!(
                    topic = %topic,
                    partition = partition,
                    offset = offset,
                    error = ?e,
                    "Invalid UTF-8 in message"
                );
                Ok(MessageAction::Reject)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_event_handler_valid() {
        let handler = UserEventHandler::new("user-events");

        let event = UserEvent {
            user_id: "user123".to_string(),
            event_type: "login".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        let payload = serde_json::to_vec(&event).unwrap();
        let result = handler
            .handle(None, &payload, "user-events", 0, 1)
            .await
            .unwrap();

        assert_eq!(result, MessageAction::Consume);
    }

    #[tokio::test]
    async fn test_user_event_handler_invalid_type() {
        let handler = UserEventHandler::new("user-events");

        let event = UserEvent {
            user_id: "user123".to_string(),
            event_type: "invalid_type".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        };

        let payload = serde_json::to_vec(&event).unwrap();
        let result = handler
            .handle(None, &payload, "user-events", 0, 1)
            .await
            .unwrap();

        assert_eq!(result, MessageAction::Reject);
    }

    #[tokio::test]
    async fn test_user_event_handler_old_event() {
        let handler = UserEventHandler::new("user-events");

        let event = UserEvent {
            user_id: "user123".to_string(),
            event_type: "login".to_string(),
            timestamp: 1000000, // Very old timestamp
        };

        let payload = serde_json::to_vec(&event).unwrap();
        let result = handler
            .handle(None, &payload, "user-events", 0, 1)
            .await
            .unwrap();

        assert_eq!(result, MessageAction::Skip);
    }

    #[tokio::test]
    async fn test_simple_string_handler() {
        let handler = SimpleStringHandler::new("messages");

        let result = handler
            .handle(None, b"Hello, World!", "messages", 0, 1)
            .await
            .unwrap();
        assert_eq!(result, MessageAction::Consume);
    }

    #[tokio::test]
    async fn test_simple_string_handler_empty() {
        let handler = SimpleStringHandler::new("messages");

        let result = handler
            .handle(None, b"   ", "messages", 0, 1)
            .await
            .unwrap();
        assert_eq!(result, MessageAction::Skip);
    }
}
