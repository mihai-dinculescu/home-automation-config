use crate::{
    db::DbPool,
    influxdb::{json_query, InfluxDbPool},
    models::{
        errors::HttpError,
        health::{HealthStatus, HealthStatusDevice, HealthStatusDeviceEnum},
    },
};
use actix_web::{
    web::{self, Json},
    Result,
};
use chrono::{DateTime, Utc};
use std::convert::Infallible;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub async fn health(
    db_pool: web::Data<DbPool>,
    influxdb_pool: web::Data<InfluxDbPool>,
) -> Result<Json<HealthStatus>, HttpError<HealthStatus>> {
    let database_postgres = web::block(move || -> Result<String, Infallible> {
        match db_pool.get() {
            Ok(_) => Ok("ok".to_string()),
            Err(e) => Ok(format!("error: {:?}", e)),
        }
    })
    .await
    .unwrap_or_else(|_| "BlockingError".to_string());

    let database_influx = web::block(move || -> Result<String, Infallible> {
        match influxdb_pool.get() {
            Ok(conn) => match conn.ping() {
                Ok(_) => Ok("ok".to_string()),
                Err(e) => Ok(format!("error: {:?}", e)),
            },
            Err(e) => Ok(format!("error: {:?}", e)),
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct ValuesCount {
    time: DateTime<Utc>,
    values_count: f64,
}

pub async fn health_devices(
    influxdb_pool: web::Data<InfluxDbPool>,
    device: web::Path<HealthStatusDeviceEnum>,
) -> Result<Json<HealthStatusDevice>, HttpError<HealthStatusDevice>> {
    let (database_influx, status) = web::block(move || -> Result<(String, String), Infallible> {
        match influxdb_pool.get() {
            Ok(connection) => {
                let (query, time) = match device.as_ref() {
                    HealthStatusDeviceEnum::Weather => {
                        let time = "3h".to_string();
                        let query = format!("SELECT COUNT(apparentTemperature) AS values_count FROM weather WHERE time > now() - {}", time);

                        (query, time)
                    }
                    HealthStatusDeviceEnum::MasterBedroom => {
                        let time = "15m".to_string();
                        let query = format!(r#"SELECT COUNT(temperature) AS values_count FROM sensors WHERE time > now() - {} AND "location" =~ /^Master Bedroom$/"#, time);

                        (query, time)
                    }
                    HealthStatusDeviceEnum::LivingRoom => {
                        let time = "15m".to_string();
                        let query = format!(r#"SELECT COUNT(temperature) AS values_count FROM sensors WHERE time > now() - {} AND "location" =~ /^Living Room$/"#, time);

                        (query, time)
                    }
                    HealthStatusDeviceEnum::Thermostat => {
                        let time = "15m".to_string();
                        let query = format!("SELECT COUNT(temperature_current) AS values_count FROM boilers WHERE time > now() - {}", time);

                        (query, time)
                    }
                };

                let result = json_query::<ValuesCount>(
                    &query,
                    &connection,
                );

                match result {
                    Ok(values ) => {
                        if values.is_empty() {
                            Ok(("ok".to_string(), format!("error: no data found in the last {}", time)))
                        } else {
                            Ok(("ok".to_string(), "ok".to_string()))
                        }
                    }
                    Err(e) => Ok((format!("error: {:?}", e), "unknown".to_string()))
                }
            }
            Err(e) => Ok((format!("error: {:?}", e), "unknown".to_string())),
        }
    })
    .await
    .unwrap_or_else(|_| ("error: BlockingError".to_string(), "unknown".to_string()));

    let status = HealthStatusDevice {
        version: VERSION.unwrap_or("").to_string(),
        database_influx,
        status,
    };

    if status.database_influx != "ok" || status.status != "ok" {
        Err(HttpError::ServiceUnavailable(status))
    } else {
        Ok(Json(status))
    }
}
