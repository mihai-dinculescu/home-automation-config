use crate::{
    db::DbPool,
    influxdb::InfluxDbPool,
    models::{errors::HttpError, health::HealthStatus},
};
use actix_web::{
    web::{self, Json},
    Result,
};
use std::convert::Infallible;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub async fn health(
    db_pool: web::Data<DbPool>,
    influxdb_pool: web::Data<InfluxDbPool>,
) -> Result<Json<HealthStatus>, HttpError<HealthStatus>> {
    let database_postgres = web::block(move || -> Result<String, Infallible> {
        match db_pool.get() {
            Ok(_) => serde::export::Ok("ok".to_string()),
            Err(e) => serde::export::Ok(format!("error: {:?}", e)),
        }
    })
    .await
    .unwrap_or_else(|_| "BlockingError".to_string());

    let database_influx = web::block(move || -> Result<String, Infallible> {
        match influxdb_pool.get() {
            Ok(c) => match c.ping() {
                Ok(_) => serde::export::Ok("ok".to_string()),
                Err(e) => serde::export::Ok(format!("error: {:?}", e)),
            },
            Err(e) => serde::export::Ok(format!("error: {:?}", e)),
        }
    })
    .await
    .unwrap_or_else(|_| "BlockingError".to_string());

    let status = HealthStatus {
        version: VERSION.unwrap_or("").to_string(),
        api: "ok".to_string(),
        database_postgres: database_postgres.clone(),
        database_influx: database_influx.clone(),
    };

    if database_postgres != "ok" || database_influx != "ok" {
        Err(HttpError::ServiceUnavailable(status))
    } else {
        Ok(Json(status))
    }
}
