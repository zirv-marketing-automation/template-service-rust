use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use common::config::LoggingConfig;
use common::logging::init_logging;
use config::register_configs;
use controllers::base::{health_check, not_found};
use zirv_config::read_config;
use zirv_db_sqlx::{get_db_pool, init_db_pool};

mod config;
mod controllers;
mod models;
mod router;
mod seeder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    register_configs();

    // Initialize structured logging for Kibana
    let logging_config = read_config!("logging", LoggingConfig).unwrap();
    init_logging(
        &logging_config.service_name,
        &logging_config.environment,
        &logging_config.level,
        &logging_config.format,
    )
    .expect("Failed to initialize logging");

    init_db_pool!();

    let _pool = get_db_pool!();

    // Migrate the database
    // Note: Migrations are disabled until migration files are created
    // Uncomment when migrations are added to ../../../migrations directory
    // tracing::info!("Running database migrations");
    // sqlx::migrate!("../../../migrations")
    //     .run(pool)
    //     .await
    //     .expect("Failed to run migrations");
    // tracing::info!("Database migrations completed");

    // Seed the database
    tracing::info!("Seeding database");
    match seeder::seed_database().await {
        | Ok(_) => tracing::info!("Database seeded successfully"),
        | Err(e) => tracing::error!(error = ?e, "Failed to seed database"),
    };

    let host = read_config!("app.host", String).unwrap();
    let port = read_config!("app.port", u16).unwrap();

    // Start Actix Web Server
    let addr = format!("{}:{}", host, port);
    tracing::info!(address = %addr, "Starting HTTP server");

    HttpServer::new(move || {
        // Configure CORS to allow only localhost
        let cors = Cors::default()
            .allowed_origin("http://localhost")
            .allowed_origin_fn(|origin, _req_head| {
                // Allow requests from localhost with any port
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(cors)
            .service(health_check)
            .service(router::get())
            .default_service(web::route().to(not_found))
    })
    .bind((host, port))?
    .run()
    .await
}
