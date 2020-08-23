#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQlErrorLocation {
    pub line: i32,
    pub column: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Vec<GraphQlErrorLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLErrors {
    pub errors: Vec<GraphQLError>,
}

impl GraphQLErrors {
    pub fn new(message: &str) -> GraphQLErrors {
        GraphQLErrors {
            errors: vec![GraphQLError {
                message: message.to_string(),
                locations: Vec::new(),
            }],
        }
    }
}

use actix_web::error;
use actix_web::HttpResponse;
use serde::Serialize;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError<T>
where
    T: Serialize + Debug,
{
    #[error("Service Unavailable")]
    ServiceUnavailable(T),
}

impl<T> error::ResponseError for HttpError<T>
where
    T: Serialize + Debug,
{
    fn error_response(&self) -> HttpResponse {
        match self {
            HttpError::ServiceUnavailable(err) => HttpResponse::ServiceUnavailable()
                .content_type("application/json")
                .body(serde_json::to_string(err).unwrap()),
        }
    }
}
