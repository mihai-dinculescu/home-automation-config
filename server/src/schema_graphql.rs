use std::sync::Arc;

use diesel::PgConnection;
use juniper::FieldResult;
use juniper::RootNode;

use crate::db::DbPooledConnection;
use crate::influxdb::InfluxDbPooledConnection;
use crate::models::thermostat_command::{ThermostatCommand, ThermostatCommandInput};
use crate::models::thermostat_status::{NewThermostatStatus, ThermostatStatus};

#[derive(Clone)]
pub struct Context {
    pub db: Arc<DbPooledConnection>,
    pub influxdb: Arc<InfluxDbPooledConnection>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "Query the current (latest) thermostat status")]
    fn thermostat_status(context: &Context) -> FieldResult<ThermostatStatus> {
        let connection: &PgConnection = &context.db;

        let result = ThermostatStatus::get_latest(connection)?;
        return Ok(result);
    }

    #[graphql(description = "Query the thermostat status history")]
    fn thermostat_status_history(context: &Context) -> FieldResult<Vec<ThermostatStatus>> {
        let connection: &PgConnection = &context.db;

        let results = ThermostatStatus::get_history(connection)?;
        return Ok(results);
    }

    #[graphql(description = "Get the next command that should be executed by the thermostat")]
    fn thermostat_command(
        context: &Context,
        data: ThermostatCommandInput,
    ) -> FieldResult<ThermostatCommand> {
        let connection = &context.influxdb;

        let result = ThermostatCommand::get_next_command(connection, data)?;
        return Ok(result);
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Set the thermostat status")]
    fn set_thermostat_status(
        context: &Context,
        data: NewThermostatStatus,
    ) -> FieldResult<ThermostatStatus> {
        let connection: &PgConnection = &context.db;

        ThermostatStatus::insert(connection, data)?;

        let result = ThermostatStatus::get_latest(connection)?;
        return Ok(result);
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {})
}

pub fn create_context(db: DbPooledConnection, influxdb: InfluxDbPooledConnection) -> Context {
    Context {
        db: Arc::new(db),
        influxdb: Arc::new(influxdb),
    }
}
