use async_trait::async_trait;

/// Result type for message handler operations
pub type HandlerResult = Result<MessageAction, Box<dyn std::error::Error + Send + Sync>>;

/// Defines the action to take after processing a message
#[derive(Debug, Clone, PartialEq)]
pub enum MessageAction {
    /// Consume and commit the message
    Consume,
    /// Skip the message without committing (will retry)
    Skip,
    /// Reject the message and commit (will not retry)
    Reject,
}

/// Trait for handling messages from Kafka topics
///
/// Implement this trait to define custom logic for processing messages from specific topics.
/// The handler can decide whether to consume, skip, or reject messages, and can perform
/// any necessary data transformation or business logic.
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Returns the topic this handler is registered for
    fn topic(&self) -> &str;

    /// Process a message and return the action to take
    ///
    /// # Arguments
    /// * `key` - Optional message key as bytes
    /// * `payload` - Message payload as bytes
    /// * `topic` - Topic the message was received from
    /// * `partition` - Partition the message was received from
    /// * `offset` - Offset of the message
    ///
    /// # Returns
    /// A `HandlerResult` indicating whether to consume, skip, or reject the message
    async fn handle(
        &self,
        key: Option<&[u8]>,
        payload: &[u8],
        topic: &str,
        partition: i32,
        offset: i64,
    ) -> HandlerResult;

    /// Optional: Transform the message before processing
    /// Default implementation returns the payload as-is
    fn transform(
        &self,
        payload: &[u8],
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(payload.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            payload: &[u8],
            _topic: &str,
            _partition: i32,
            _offset: i64,
        ) -> HandlerResult {
            // Simple test: consume if payload is not empty
            if payload.is_empty() {
                Ok(MessageAction::Skip)
            } else {
                Ok(MessageAction::Consume)
            }
        }
    }

    #[tokio::test]
    async fn test_message_handler_consume() {
        let handler = TestHandler { topic: "test-topic".to_string() };
        assert_eq!(handler.topic(), "test-topic");

        let result = handler
            .handle(None, b"test payload", "test-topic", 0, 1)
            .await
            .unwrap();
        assert_eq!(result, MessageAction::Consume);
    }

    #[tokio::test]
    async fn test_message_handler_skip() {
        let handler = TestHandler { topic: "test-topic".to_string() };

        let result = handler.handle(None, b"", "test-topic", 0, 1).await.unwrap();
        assert_eq!(result, MessageAction::Skip);
    }

    #[test]
    fn test_transform_default() {
        let handler = TestHandler { topic: "test-topic".to_string() };

        let payload = b"test data";
        let result = handler.transform(payload).unwrap();
        assert_eq!(result, payload.to_vec());
    }
}
