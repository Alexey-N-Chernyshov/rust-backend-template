use actix_http::Request;
use actix_service::Service;
use actix_web::dev::ServiceResponse;
use actix_web::test::TestRequest;
use actix_web::{test, web, App};
use my_project_name::config::ServerConfig;
use my_project_name::db;
use my_project_name::db::{create_connection_pool, Pool};
use my_project_name::metrics::Metrics;
use my_project_name::model::{AppVersion, HealthCheck};
use my_project_name::routes::init_routes;
use my_project_name::server_state::ServerState;
use std::sync::Arc;
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

/// Integration tests. Uses Postgres image.

struct TestFixture {
    config: ServerConfig,
    db_connection_pool: Pool,
    _postgres_image: ContainerAsync<GenericImage>,
}

impl TestFixture {
    /// Sets up testing environment.
    /// Creates Postgres database and ServerState.
    pub async fn setup() -> Self {
        let db = "postgres-db-test";
        let user = "postgres-user-test";
        let password = "postgres-password-test";

        let generic_postgres = GenericImage::new("postgres", "14")
            .with_exposed_port(ContainerPort::Tcp(5432))
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", db)
            .with_env_var("POSTGRES_USER", user)
            .with_env_var("POSTGRES_PASSWORD", password);

        let postgres = generic_postgres.start().await.expect("Must start postrges");

        let connection_string = &format!(
            "postgres://{}:{}@127.0.0.1:{}/{}",
            user,
            password,
            postgres
                .get_host_port_ipv4(5432)
                .await
                .expect("Must return port"),
            db
        );

        let db_connection_pool = create_connection_pool(connection_string);
        db::init(connection_string.as_str());

        let config = ServerConfig {};

        Self {
            config,
            db_connection_pool,
            _postgres_image: postgres,
        }
    }

    /// Starts backend service.
    pub async fn start_backend(
        &self,
    ) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
        test::init_service(
            App::new()
                .app_data(web::Data::new(ServerState::new(
                    self.db_connection_pool.clone(),
                    Arc::new(Metrics::default()),
                    self.config.clone(),
                )))
                .configure(init_routes),
        )
        .await
    }

    pub fn build_request_get_health(&self) -> Request {
        TestRequest::get().uri("/health").to_request()
    }

    pub fn build_request_get_version(&self) -> Request {
        TestRequest::get().uri("/version").to_request()
    }
}

#[actix_web::test]
async fn test_healthcheck() {
    let test = TestFixture::setup().await;
    let backend = test.start_backend().await;

    let request = test.build_request_get_health();
    let response = test::call_service(&backend, request).await;
    assert_eq!(response.status(), 200);
    let bytes = test::read_body(response).await;
    let healthcheck: HealthCheck = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        healthcheck,
        HealthCheck {
            healthy: true,
            db_connected: true,
            description: None,
        }
    );
}

#[actix_web::test]
async fn test_version() {
    let test = TestFixture::setup().await;
    let backend = test.start_backend().await;

    let request = test.build_request_get_version();
    let response = test::call_service(&backend, request).await;
    assert_eq!(response.status(), 200);
    let bytes = test::read_body(response).await;
    let healthcheck: AppVersion = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(healthcheck, AppVersion::new(),);
}
