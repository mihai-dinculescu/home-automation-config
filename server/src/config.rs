use crate::models::key::Key;
use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub key: Key,
    pub postgres_db_url: String,
    pub influxdb_host: String,
    pub influxdb_db: String,
    pub influxdb_username: String,
    pub influxdb_password: String,
}

#[derive(PartialEq)]
pub enum ConfigEnvironmentEnum {
    Production,
    Development,
    Test,
}

impl Config {
    pub fn new(config_env: ConfigEnvironmentEnum) -> Self {
        // load .env variables
        if config_env == ConfigEnvironmentEnum::Test {
            dotenv::from_filename(".test.env").ok();
        }

        dotenv().ok();

        let get_var =
            |name: &str| env::var(name).unwrap_or_else(|_| panic!("{} must be set", name));

        Self {
            host: env::var("HOST").expect("Missing `HOST` env variable"),
            port: env::var("PORT").expect("Missing `PORT` env variable"),
            key: Key::new(env::var("API_KEY").expect("Missing `API_KEY` env variable")),
            postgres_db_url: format!(
                "postgres://{}:{}@{}/{}",
                get_var("POSTGRES_USER"),
                get_var("POSTGRES_PASSWORD"),
                get_var("POSTGRES_DB_HOST"),
                get_var("POSTGRES_DB")
            ),
            influxdb_host: get_var("INFLUXDB_HOST"),
            influxdb_db: get_var("INFLUXDB_DB"),
            influxdb_username: get_var("INFLUXDB_USERNAME"),
            influxdb_password: get_var("INFLUXDB_PASSWORD"),
        }
    }
}
