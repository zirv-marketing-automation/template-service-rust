use common::config::LoggingConfig;
use common::logging::init_logging;
use kafka_messages::messages::TemplateMessageHandler;
use kafka_messages::MessageHandler;
use std::sync::Arc;
use tokio::signal;
use zirv_config::{read_config, register_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Register configurations
    register_config!("logging", LoggingConfig::default());

    // Initialize structured logging for Kibana
    let logging_config = read_config!("logging", LoggingConfig).unwrap();
    init_logging(
        &logging_config.service_name,
        &logging_config.environment,
        &logging_config.level,
        &logging_config.format,
    )
    .expect("Failed to initialize logging");

    tracing::info!("Starting Kafka consumer service");

    // Initialize handlers - each handler is coupled with a topic
    let handlers: Vec<Arc<dyn MessageHandler>> = vec![
        Arc::new(TemplateMessageHandler),
        // Add more handlers here as needed
        // Each handler implements the MessageHandler trait and specifies its topic
    ];

    tracing::info!("Registered {} message handlers", handlers.len());
    for handler in &handlers {
        tracing::info!("Handler registered for topic: {}", handler.topic());
    }

    // TODO: Integrate with zirv-kafka when librdkafka is available
    // The integration would look like:
    // 1. Create KafkaConsumer with configuration
    // 2. Subscribe to topics from all handlers
    // 3. Poll for messages and dispatch to appropriate handler based on topic
    // 
    // Example integration (requires zirv-kafka):
    // let kafka_config = read_config!("kafka", KafkaConfig).unwrap();
    // let consumer = KafkaConsumer::new(&kafka_config)?;
    // let topics: Vec<&str> = handlers.iter().map(|h| h.topic()).collect();
    // consumer.subscribe(&topics)?;
    //
    // loop {
    //     match consumer.poll() {
    //         Ok(Some(message)) => {
    //             let topic = message.topic();
    //             let payload = message.payload_str()?;
    //             if let Some(handler) = handlers.iter().find(|h| h.topic() == topic) {
    //                 handler.handle(payload).await?;
    //             }
    //         }
    //         Ok(None) => tokio::time::sleep(Duration::from_millis(100)).await,
    //         Err(e) => tracing::error!("Error polling Kafka: {:?}", e),
    //     }
    // }

    tracing::info!("Kafka consumer service started (demo mode - awaiting zirv-kafka integration)");
    tracing::info!("To integrate with actual Kafka:");
    tracing::info!("1. Install librdkafka system library");
    tracing::info!("2. Uncomment zirv-kafka dependency in Cargo.toml");
    tracing::info!("3. Uncomment the kafka integration code in main.rs");

    // Wait for shutdown signal
    signal::ctrl_c().await?;
    
    tracing::info!("Shutdown signal received, stopping kafka consumer");
    
    Ok(())
}
