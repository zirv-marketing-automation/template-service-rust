# Service Architecture

This template service has been refactored to support multiple containerized services with shared common logic.

## Structure

```
template-service-rust/
├── common/                  # Shared library across all services
│   ├── src/
│   │   ├── config/         # Common configuration (logging, etc.)
│   │   ├── logging.rs      # Logging utilities
│   │   └── utils/          # Utility functions
│   └── Cargo.toml
├── services/               # Individual services
│   ├── api/               # REST API service
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── Dockerfile
│   └── kafka/             # Kafka consumer service
│       ├── src/
│       ├── Cargo.toml
│       └── Dockerfile
├── kafka-messages/        # Kafka message definitions and handlers
│   ├── src/
│   │   ├── lib.rs
│   │   └── messages.rs
│   └── Cargo.toml
├── migrations/            # Database migrations (shared)
├── docker-compose.yml     # Multi-service orchestration
└── Cargo.toml            # Workspace configuration
```

## Services

### API Service (`services/api`)
- REST API built with Actix Web
- Handles HTTP requests
- Uses SQLx for database access
- Containerized independently

### Kafka Consumer Service (`services/kafka`)
- Consumes messages from Kafka topics
- Uses `zirv-kafka` package for Kafka integration
- Processes messages using handlers from `kafka-messages`
- Containerized independently

## Common Library

The `common` library provides shared functionality:
- **Logging Configuration**: Structured logging setup for Kibana
- **Logging Utilities**: JSON and pretty logging formats
- **Utility Functions**: Environment variable helpers and other common utilities

## Kafka Messages

The `kafka-messages` library defines:
- **MessageHandler Trait**: All Kafka message handlers must implement this trait
- **Message Structs**: Serde-compatible message definitions
- **Handler Implementations**: Business logic for processing messages

### Adding a New Kafka Message Handler

1. Define your message struct in `kafka-messages/src/messages.rs`:
```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyNewMessage {
    pub id: String,
    pub data: String,
}
```

2. Create a handler implementing `MessageHandler`:
```rust
pub struct MyNewMessageHandler;

#[async_trait]
impl MessageHandler for MyNewMessageHandler {
    async fn handle(&self, payload: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message: MyNewMessage = serde_json::from_str(payload)?;
        // Your business logic here
        Ok(())
    }
    
    fn topic(&self) -> &str {
        "my.new.topic"
    }
}
```

3. Register the handler in `services/kafka/src/main.rs`:
```rust
let handlers: Vec<Arc<dyn MessageHandler>> = vec![
    Arc::new(TemplateMessageHandler),
    Arc::new(MyNewMessageHandler),  // Add your handler here
];
```

## Building and Running

### Build All Services
```bash
cargo build --release
```

### Build Individual Services
```bash
cargo build -p api --release
cargo build -p kafka --release
```

### Run with Docker Compose
```bash
docker-compose up -d
```

This will start:
- API service on port 3000
- Kafka consumer service
- MySQL database on port 3306
- Kafka on port 9092
- Zookeeper

### Run Individually (Development)
```bash
# API Service
cd services/api
cargo run

# Kafka Consumer Service
cd services/kafka
cargo run
```

## Testing

```bash
# Run all tests
cargo test

# Test specific package
cargo test -p common
cargo test -p api
cargo test -p kafka
cargo test -p kafka-messages
```

## Configuration

Services are configured via environment variables:

### Common Configuration
- `LOG_LEVEL`: Logging level (trace, debug, info, warn, error)
- `LOG_FORMAT`: Log format (json, pretty)
- `SERVICE_NAME`: Service identifier
- `ENVIRONMENT`: Environment name (production, staging, development)

### API Service
- `HOST`: Bind address (default: 0.0.0.0)
- `PORT`: Port number (default: 3000)
- `DATABASE_URL`: Database connection string
- `MAX_DATABASE_CONNECTIONS`: Max DB connections (default: 5)

### Kafka Consumer Service
- `KAFKA_BROKERS`: Kafka broker addresses
- `KAFKA_GROUP_ID`: Consumer group ID

## Integration with zirv-kafka

To enable full Kafka integration:

1. Install librdkafka system library:
```bash
# Debian/Ubuntu
sudo apt-get install librdkafka-dev

# macOS
brew install librdkafka
```

2. Uncomment `zirv-kafka` dependency in `services/kafka/Cargo.toml`

3. Uncomment Kafka integration code in `services/kafka/src/main.rs`

4. Rebuild and run:
```bash
cargo build -p kafka --release
```

## Notes

- Each service is containerized separately for independent scaling
- The `common` library ensures consistency across services
- Kafka topics are coupled with message structs that implement the `MessageHandler` trait
- All services use structured logging for easy integration with Kibana
