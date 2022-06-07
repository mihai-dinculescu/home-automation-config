use crate::influxdb_pool::{AuthInfo, InfluxDBConnectionManager};
use influxdb::{integrations::serde_integration::DatabaseQueryResult, Error, Query};
use serde::Deserialize;

pub type InfluxDbPool = bb8::Pool<InfluxDBConnectionManager>;
pub type InfluxDbPooledConnection<'a> = bb8::PooledConnection<'a, InfluxDBConnectionManager>;

async fn init_pool(info: AuthInfo) -> Result<InfluxDbPool, Error> {
    let manager = InfluxDBConnectionManager::new(info);
    bb8::Pool::builder().build(manager).await
}

pub async fn establish_connection(
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

    init_pool(info)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
}

pub async fn json_query<'a, DATA>(
    query: &'a str,
    connection: &'a InfluxDbPooledConnection<'a>,
) -> anyhow::Result<Vec<DATA>>
where
    DATA: for<'de> Deserialize<'de> + std::marker::Send + Clone + 'static,
{
    let read_query = <dyn Query>::raw_read_query(query);
    let read_result = connection.query(&read_query).await?;

    let result = serde_json::from_slice::<DatabaseQueryResult>(read_result.as_bytes())
        .map_err(|err| anyhow::anyhow!("Influx query error: {}", err))
        .and_then(|mut db_result: DatabaseQueryResult| {
            db_result
                .deserialize_next::<DATA>()
                .map_err(|err| anyhow::anyhow!("Influx deserialize error: {}", err))
        });

    let result = result?.series[0].values.clone();

    Ok(result)
}

pub async fn json_query_tagged<'a, DATA, TAGS>(
    query: &'a str,
    connection: &'a InfluxDbPooledConnection<'a>,
) -> anyhow::Result<Vec<DATA>>
where
    DATA: for<'de> Deserialize<'de> + std::marker::Send + Clone + 'static,
    TAGS: for<'de> Deserialize<'de> + std::marker::Send + Clone + 'static,
{
    let read_query = <dyn Query>::raw_read_query(query);
    let read_result = connection.query(&read_query).await?;

    let result = serde_json::from_slice::<DatabaseQueryResult>(read_result.as_bytes())
        .map_err(|err| anyhow::anyhow!("Influx query error: {}", err))
        .and_then(|mut db_result: DatabaseQueryResult| {
            db_result
                .deserialize_next_tagged::<TAGS, DATA>()
                .map_err(|err| anyhow::anyhow!("Influx deserialize error: {}", err))
        });

    let result = result?.series[0].values.clone();

    Ok(result)
}
