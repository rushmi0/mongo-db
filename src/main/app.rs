use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use actix_web::http::header;
use dotenvy::dotenv;
use std::env;
use actix_web::web::Data;
use crate::services::api::v1;
use crate::storage;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    storage::init_db().await;
    env_logger::init();

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("APP_PORT must be a valid u16");

    let db_client = storage::init_db().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors_config())
            .app_data(Data::new(db_client.clone()))
            .configure(v1::service_hub)
    })
        .bind((host.as_str(), port))?
        .run()
        .await
}

fn cors_config() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3700)
        .send_wildcard()
        .allow_any_origin()
}