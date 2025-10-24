use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod messages;

/// Trait that all kafka message handlers must implement
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handle the incoming message
    async fn handle(&self, payload: &str) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get the topic this handler is responsible for
    fn topic(&self) -> &str;
}

/// Example template message
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateMessage {
    pub id: String,
    pub content: String,
    pub timestamp: i64,
}
