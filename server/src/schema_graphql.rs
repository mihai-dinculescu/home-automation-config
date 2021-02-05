use std::sync::Arc;

use juniper::{graphql_object, RootNode};
use juniper::{EmptySubscription, FieldResult};

use crate::db::DbPool;
use crate::influxdb::InfluxDbPool;
use crate::models::thermostat_command::{ThermostatCommand, ThermostatCommandInput};
use crate::models::thermostat_status::{NewThermostatStatus, ThermostatStatus};

#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<DbPool>,
    pub influxdb_pool: Arc<InfluxDbPool>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Query the current (latest) thermostat status")]
    fn thermostat_status(context: &Context) -> FieldResult<ThermostatStatus> {
        let connection = &context.db_pool.get()?;

        let result = ThermostatStatus::get_latest(connection)?;
        Ok(result)
    }

    #[graphql(description = "Query the thermostat status history")]
    fn thermostat_status_history(context: &Context) -> FieldResult<Vec<ThermostatStatus>> {
        let connection = &context.db_pool.get()?;

        let results = ThermostatStatus::get_history(connection)?;
        Ok(results)
    }

    #[graphql(description = "Get the next command that should be executed by the thermostat")]
    fn thermostat_command(
        context: &Context,
        data: ThermostatCommandInput,
    ) -> FieldResult<ThermostatCommand> {
        let connection = &context.influxdb_pool.get()?;

        let result = ThermostatCommand::get_next_command(connection, data)?;
        Ok(result)
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    #[graphql(description = "Set the thermostat status")]
    fn set_thermostat_status(
        context: &Context,
        data: NewThermostatStatus,
    ) -> FieldResult<ThermostatStatus> {
        let connection = &context.db_pool.get()?;

        ThermostatStatus::insert(connection, data)?;

        let result = ThermostatStatus::get_latest(connection)?;
        Ok(result)
    }
}

pub type SchemaGraphQL = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> SchemaGraphQL {
    SchemaGraphQL::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn create_context(db_pool: Arc<DbPool>, influxdb_pool: Arc<InfluxDbPool>) -> Context {
    Context {
        db_pool,
        influxdb_pool,
    }
}
