use crate::server_state::ServerState;
use actix_web::web;
use diesel::{sql_query, RunQueryDsl};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// General API response
#[derive(Deserialize, Serialize, ToSchema)]
pub struct Response<T> {
    /// API version
    pub api_version: String,
    /// Response status, 0 - Ok, else error
    pub status: i32,
    /// Response description, in case of error contains detailed description.
    pub description: Option<String>,
    /// Response value.
    pub value: T,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, PartialEq)]
pub struct HealthCheck {
    pub healthy: bool,
    pub db_connected: bool,
    pub description: Option<String>,
}

impl HealthCheck {
    pub fn new(server_state: web::Data<ServerState>) -> HealthCheck {
        match server_state.db_connection() {
            Ok(mut connection) => match sql_query("SELECT 1").execute(&mut connection) {
                Ok(_) => HealthCheck {
                    healthy: true,
                    db_connected: true,
                    description: None,
                },
                Err(error) => HealthCheck {
                    healthy: false,
                    db_connected: false,
                    description: Some(error.to_string()),
                },
            },
            Err(error) => HealthCheck {
                healthy: false,
                db_connected: false,
                description: Some(error.to_string()),
            },
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, PartialEq)]
pub struct AppVersion {
    pub api_version: String,
    pub build_timestamp: String,
    pub git_pretty: String,
}

impl AppVersion {
    pub fn new() -> Self {
        Self {
            api_version: env!("CARGO_PKG_VERSION").to_string(),
            build_timestamp: env!("BUILD_TIMESTAMP").to_string(),
            git_pretty: env!("GIT_PRETTY_VERSION").to_string(),
        }
    }
}
