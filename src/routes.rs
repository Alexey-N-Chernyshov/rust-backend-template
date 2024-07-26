use crate::model::{AppVersion, HealthCheck};
use crate::server_state::ServerState;
use actix_web::{get, web, HttpResponse};
use utoipa::openapi::Info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(health, version,), components(schemas(AppVersion, HealthCheck,)))]
struct ApiDoc;

/// Healthcheck
#[utoipa::path(
    tag = "System",
    get,
    path = "/health",
    responses(
        (status = 200, description = "Healthcheck", body = HealthCheck),
        (status = 500, description = "No DB connection", body = HealthCheck),
    ),
)]
#[get("/health")]
async fn health(server_state: web::Data<ServerState>) -> HttpResponse {
    let healthcheck = HealthCheck::new(server_state);
    let mut status_code = if healthcheck.db_connected {
        HttpResponse::Ok()
    } else {
        HttpResponse::ServiceUnavailable()
    };
    status_code.json(healthcheck)
}

/// Returns version
#[utoipa::path(
    tag = "System",
    get,
    path = "/version",
    responses(
        (status = 200, description = "Version", body = AppVersion),
    ),
)]
#[get("/version")]
async fn version() -> HttpResponse {
    HttpResponse::Ok().json(AppVersion::new())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(health);
    config.service(version);

    let mut open_api = ApiDoc::openapi();
    open_api.info = Info::new("my-project-name", env!("GIT_PRETTY_VERSION"));
    config.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", open_api));
}
