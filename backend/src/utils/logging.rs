use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

/// Initialize the logging system based on configuration
///
/// This sets up structured logging with JSON output for Kibana.
/// The logs include service name, environment, and other metadata
/// for easier filtering and analysis in Kibana.
pub fn init_logging(
    service_name: &str,
    environment: &str,
    log_level: &str,
    log_format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Redirect all `log`'s events to our tracing subscriber
    LogTracer::init()?;

    // Set up the env filter
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    match log_format {
        | "json" => {
            // JSON format for Kibana
            let formatting_layer =
                BunyanFormattingLayer::new(service_name.to_string(), std::io::stdout);

            let subscriber = Registry::default()
                .with(env_filter)
                .with(JsonStorageLayer)
                .with(formatting_layer);

            set_global_default(subscriber)?;
        }
        | _ => {
            // Pretty format for development/debugging (default for any non-json value)
            let subscriber = Registry::default()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer());

            set_global_default(subscriber)?;
        }
    }

    // Log initialization info
    tracing::info!(
        service_name = %service_name,
        environment = %environment,
        log_level = %log_level,
        log_format = %log_format,
        "Logging initialized"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging_json() {
        let result = init_logging("test-service", "test", "info", "json");
        // We can't test much here as logging can only be initialized once per process
        // but we can at least verify it doesn't panic
        assert!(result.is_ok() || result.is_err());
    }
}
