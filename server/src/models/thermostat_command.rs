use juniper::{FieldResult, GraphQLInputObject, GraphQLObject};

use crate::influxdb::{json_query_tagged, InfluxDbPooledConnection};
use chrono::{DateTime, Utc};

#[derive(GraphQLObject)]
#[graphql(description = "The next command that should be executed by the thermostat")]
pub struct ThermostatCommand {
    pub status: bool,
    pub outside_temperature: f64,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Thermostat command input parameters")]
pub struct ThermostatCommandInput {
    pub current_temperature: f64,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
struct Weather {
    time: DateTime<Utc>,
    apparent_temperature: f64,
}
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
struct WeatherTags {
    url: String,
}

impl ThermostatCommand {
    pub async fn get_next_command<'a>(
        connection: &'a InfluxDbPooledConnection<'a>,
        _data: ThermostatCommandInput,
    ) -> FieldResult<ThermostatCommand> {
        let result = json_query_tagged::<Weather, WeatherTags>(
            "SELECT apparentTemperature AS apparent_temperature FROM weather GROUP BY * ORDER BY DESC LIMIT 1",
            connection,
        ).await?;
        let result = &result[0];

        Ok(ThermostatCommand {
            status: true,
            outside_temperature: result.apparent_temperature,
        })
    }
}
