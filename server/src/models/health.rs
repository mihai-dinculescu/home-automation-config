use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HealthStatus {
    pub version: String,
    pub api: String,
    pub database_postgres: String,
    pub database_influx: String,
}
