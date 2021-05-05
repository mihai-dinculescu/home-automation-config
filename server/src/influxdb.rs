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

pub fn establish_connection(
    url: &str,
    database: &str,
    username: &str,
    password: &str,
) -> InfluxDbPool {
    let info = AuthInfo {
        url: url.to_string(),
        database: database.to_string(),
        username: username.to_string(),
        password: password.to_string(),
    };

    init_pool(info).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

pub fn json_query<'a, T>(
    query: &'a str,
    connection: &'a InfluxDbPooledConnection,
) -> anyhow::Result<Vec<T>>
where
    T: for<'de> Deserialize<'de> + std::marker::Send + Clone + 'static,
{
    let read_query = <dyn Query>::raw_read_query(query);
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
