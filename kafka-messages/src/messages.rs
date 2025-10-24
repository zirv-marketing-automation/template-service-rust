use crate::{MessageHandler, TemplateMessage};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Example message for template topic
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExampleTemplateMessage {
    #[serde(flatten)]
    pub inner: TemplateMessage,
}

/// Handler for template messages
pub struct TemplateMessageHandler;

#[async_trait]
impl MessageHandler for TemplateMessageHandler {
    async fn handle(&self, payload: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message: ExampleTemplateMessage = serde_json::from_str(payload)?;

        tracing::info!(
            message_id = %message.inner.id,
            content = %message.inner.content,
            timestamp = %message.inner.timestamp,
            "Processing template message"
        );

        // Add your business logic here
        // For example: save to database, trigger workflow, etc.

        Ok(())
    }

    fn topic(&self) -> &str {
        "template.events"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_template_message_handler() {
        let handler = TemplateMessageHandler;
        let payload = r#"{"id":"123","content":"test","timestamp":1234567890}"#;

        let result = handler.handle(payload).await;
        assert!(result.is_ok());
    }
}
