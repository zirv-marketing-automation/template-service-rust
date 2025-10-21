use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use config::register_configs;
use controllers::base::{health_check, not_found};
use zirv_config::read_config;
use zirv_db_sqlx::{get_db_pool, init_db_pool};

mod config;
mod controllers;
mod models;
mod router;
mod seeder;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    register_configs();

    env_logger::init();

    init_db_pool!();

    let pool = get_db_pool!();

    // Migrate the database
    sqlx::migrate!("../migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");

    // Seed the database
    match seeder::seed_database().await {
        | Ok(_) => println!("Database seeded successfully."),
        | Err(e) => eprintln!("Failed to seed database: {:?}", e),
    };

    let host = read_config!("app.host", String).unwrap();
    let port = read_config!("app.port", u16).unwrap();

    // Start Actix Web Server
    let addr = format!("{}:{}", host, port);
    println!("Server running at http://{}", addr);

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
            .wrap(Logger::default())
            .wrap(cors)
            .service(health_check)
            .service(router::get())
            .default_service(web::route().to(not_found))
    })
    .bind((host, port))?
    .run()
    .await
}
