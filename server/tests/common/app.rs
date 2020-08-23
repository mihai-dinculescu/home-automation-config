use actix_http::error::Error;
use actix_http::Request;
use actix_web::dev::Service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, web, App, HttpResponse};

use diesel_migrations::{revert_latest_migration, run_pending_migrations};

use lib::db;
use lib::handlers::{
    graphql::{graphql, playground},
    health::health,
};
use lib::influxdb;
use lib::{
    config::{Config, ConfigEnvironmentEnum},
    schema_graphql::create_schema,
};

pub async fn create_app(
) -> impl Service<Request = Request, Response = ServiceResponse, Error = Error> {
    let config = Config::new(ConfigEnvironmentEnum::Test);
    let key = config.key;

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
    );

    // get connection
    let connection = db_pool.get().unwrap();

    // revert migration
    // this works currently only because there is a single migration
    // will need to figure out how to do this for multiple migrations
    revert_latest_migration(&connection).ok();

    // run pending migrations
    run_pending_migrations(&connection).expect("run pending migrations error");

    // http test server
    test::init_service(
        App::new()
            .data(db_pool.clone())
            .data(influxdb_pool.clone())
            .data(schema.clone())
            .data(key.clone())
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
            .service(web::resource("/health").route(web::get().to(health)))
            .default_service(web::route().to(|| {
                HttpResponse::Found()
                    .header("location", "/playground")
                    .finish()
            })),
    )
    .await
}
