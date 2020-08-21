use dotenv::dotenv;
use std::env;

use influxdb::integrations::serde_integration::DatabaseQueryResult;
use r2d2_influxdb::influxdb::Query;
use r2d2_influxdb::r2d2;
use r2d2_influxdb::{AuthInfo, InfluxDBConnectionManager};
use serde::Deserialize;

pub type InfluxDbPool = r2d2::Pool<InfluxDBConnectionManager>;
pub type InfluxDbPooledConnection = r2d2::PooledConnection<InfluxDBConnectionManager>;

fn init_pool(info: AuthInfo) -> Result<InfluxDbPool, r2d2::Error> {
    let manager = InfluxDBConnectionManager::new(info);
    r2d2::Pool::builder().build(manager)
}

pub fn establish_connection() -> InfluxDbPool {
    dotenv().ok();

    let influxdb_host = env::var("INFLUXDB_HOST").expect("INFLUXDB_HOST must be set");
    let influxdb_db = env::var("INFLUXDB_DB").expect("INFLUXDB_DB must be set");
    let influxdb_username = env::var("INFLUXDB_USERNAME").expect("INFLUXDB_USERNAME must be set");
    let influxdb_password = env::var("INFLUXDB_PASSWORD").expect("INFLUXDB_PASSWORD must be set");

    let info = AuthInfo {
        url: influxdb_host.clone(),
        database: influxdb_db,
        username: influxdb_username,
        password: influxdb_password,
    };

    init_pool(info).unwrap_or_else(|_| panic!("Error connecting to {}", influxdb_host))
}

pub fn json_query<'a, T>(
    query: &'a str,
    connection: &'a InfluxDbPooledConnection,
) -> anyhow::Result<Vec<T>>
where
    T: for<'de> Deserialize<'de> + std::marker::Send + Clone + 'static,
{
    let read_query = Query::raw_read_query(query);
    let read_result = connection
        .query(&read_query)
        .map_err(|err| anyhow::anyhow!("Influx connection error: {}", err))?;

    let result = serde_json::from_slice::<DatabaseQueryResult>(read_result.as_bytes())
        .map_err(|err| anyhow::anyhow!("Influx query error: {}", err))
        .and_then(|mut db_result: DatabaseQueryResult| {
            db_result
                .deserialize_next::<T>()
                .map_err(|err| anyhow::anyhow!("Influx deserialize error: {}", err))
        });

    let result = result?.series[0].values.clone();

    Ok(result)
}
