use std::io;

use actix_cors::Cors;
use actix_web::middleware::{self, errhandlers::ErrorHandlers};
use actix_web::{http, web, App, HttpResponse, HttpServer};
use diesel_migrations::run_pending_migrations;

use ::lib::db;
use ::lib::handlers::{
    error::error_handler,
    graphql::{graphql, playground},
    health::{health, health_devices},
};
use ::lib::influxdb;
use ::lib::{
    config::{Config, ConfigEnvironmentEnum},
    schema_graphql::create_schema,
};

#[cfg(debug_assertions)]
fn get_env() -> ConfigEnvironmentEnum {
    ConfigEnvironmentEnum::Development
}

#[cfg(not(debug_assertions))]
fn get_env() -> ConfigEnvironmentEnum {
    ConfigEnvironmentEnum::Production
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = Config::new(get_env());
    let key = config.key;

    // configure logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // database connection pool
    let db_pool = db::establish_connection(&config.postgres_db_url);

    // influxdb connection pool
    let influxdb_pool = influxdb::establish_connection(
        &config.influxdb_host,
        &config.influxdb_db,
        &config.influxdb_username,
        &config.influxdb_password,
    )
    .await;

    // run pending postgres migrations
    let connection = db_pool.get().unwrap();
    run_pending_migrations(&connection)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let host = format!("{}:{}", &config.host, &config.port);
    let http_host = format!("http://{}", &host);

    println!("Starting GraphQL server at {}", &http_host);

    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&http_host)
                    .allowed_methods(vec!["GET", "POST"]),
            )
            .data(db_pool.clone())
            .data(influxdb_pool.clone())
            .data(schema.clone())
            .data(key.clone())
            .wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, error_handler))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .service(web::resource("/health").route(web::get().to(health)))
            .service(web::resource("/device-health/{device}").route(web::get().to(health_devices)))
            .default_service(web::route().to(|| {
                HttpResponse::Found()
                    .header("location", "/playground")
                    .finish()
            }))
    })
    .bind(host)?
    .run()
    .await
}
