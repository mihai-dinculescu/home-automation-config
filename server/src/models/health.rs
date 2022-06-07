use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HealthStatus {
    pub version: String,
    pub api: String,
    pub database_postgres: String,
    pub database_influx: String,
}

#[derive(Deserialize)]
pub enum HealthStatusDeviceEnum {
    #[serde(rename = "weather")]
    Weather,
    #[serde(rename = "thermostat")]
    Thermostat,
    #[serde(rename = "living_room")]
    LivingRoom,
    #[serde(rename = "master_bedroom")]
    MasterBedroom,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct HealthStatusDevice {
    pub version: String,
    pub database_influx: String,
    pub status: String,
}
