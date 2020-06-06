use juniper::{FieldResult, GraphQLInputObject, GraphQLObject};

use crate::influxdb::{json_query, InfluxDbPooledConnection};

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
    time: String,
    apparent_temperature: f64,
}

impl ThermostatCommand {
    pub fn get_next_command(
        connection: &InfluxDbPooledConnection,
        _data: ThermostatCommandInput,
    ) -> FieldResult<ThermostatCommand> {
        let result = json_query::<Weather>(
            "SELECT apparentTemperature AS apparent_temperature FROM weather GROUP BY * ORDER BY DESC LIMIT 1",
            &connection,
        )?;
        let result = &result[0];

        Ok(ThermostatCommand {
            status: true,
            outside_temperature: result.apparent_temperature,
        })
    }
}
