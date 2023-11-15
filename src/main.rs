use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod data;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    println!("Starting server at: {}", &bind_address);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .configure(routes::config)
            .service(actix_web::web::scope("/navbar").configure(routes::config))
            .service(actix_web::web::scope("/health").configure(routes::config_health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}
