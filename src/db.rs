use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn create_connection_pool<S: Into<String>>(db_url: S) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Failed to create db pool")
}

pub fn init(db_url: &str) {
    let mut connection =
        PgConnection::establish(db_url).unwrap_or_else(|_| panic!("Error connecting to {db_url}"));
    connection.run_pending_migrations(MIGRATIONS).unwrap();
}
