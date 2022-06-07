// TODO: extract this into a separate crate
use async_trait::async_trait;
use influxdb::{Client, Error};

/// InfluxDB connection info with Authenticator
pub struct AuthInfo {
    pub url: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl From<AuthInfo> for InfluxDBConnectionInfo {
    fn from(item: AuthInfo) -> Self {
        InfluxDBConnectionInfo {
            url: item.url,
            database: item.database,
            username: Some(item.username),
            password: Some(item.password),
            is_auth: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InfluxDBConnectionInfo {
    url: String,
    database: String,
    username: Option<String>,
    password: Option<String>,
    is_auth: bool,
}

#[derive(Debug)]
pub struct InfluxDBConnectionManager {
    info: InfluxDBConnectionInfo,
}

impl InfluxDBConnectionManager {
    pub fn new<T: Into<InfluxDBConnectionInfo>>(params: T) -> InfluxDBConnectionManager {
        InfluxDBConnectionManager {
            info: params.into(),
        }
    }
}

#[async_trait]
impl bb8::ManageConnection for InfluxDBConnectionManager {
    type Connection = Client;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = if self.info.is_auth {
            Client::new(self.info.url.to_owned(), self.info.database.to_owned()).with_auth(
                self.info.username.to_owned().unwrap(),
                self.info.password.to_owned().unwrap(),
            )
        } else {
            Client::new(self.info.url.to_owned(), self.info.database.to_owned())
        };

        Ok(client)
    }

    async fn is_valid(
        &self,
        conn: &mut bb8::PooledConnection<'_, Self>,
    ) -> Result<(), Self::Error> {
        conn.ping().await.map(|_| ())
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
