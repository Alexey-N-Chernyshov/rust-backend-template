use crate::config::ServerConfig;
use crate::db::{DbConnection, Pool};
use crate::error_handler::CustomError;
use crate::metrics::Metrics;
use std::sync::Arc;

/// Server state and configuration
pub struct ServerState {
    // PostgreSQL connection pool
    pub db_connection_pool: Pool,
    pub metrics: Arc<Metrics>,
    pub config: ServerConfig,
}

impl ServerState {
    pub fn new(
        db_connection_pool: Pool,
        metrics: Arc<Metrics>,
        config: ServerConfig,
    ) -> ServerState {
        ServerState {
            db_connection_pool,
            metrics,
            config,
        }
    }
    /// Returns connection to DB from pool.
    pub fn db_connection(&self) -> Result<DbConnection, CustomError> {
        self.db_connection_pool
            .get()
            .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {e}")))
    }
}
