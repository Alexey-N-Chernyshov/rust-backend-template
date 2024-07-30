use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use log::info;
use my_project_name::config::ServerConfig;
use my_project_name::db::create_connection_pool;
use my_project_name::metrics::Metrics;
use my_project_name::routes::init_routes;
use my_project_name::server_state::ServerState;
use my_project_name::{db, error_handler};
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Start my-project-name {}", env!("GIT_PRETTY_VERSION"));

    let config = ServerConfig::new_from_envs();

    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    let db_connection_pool = create_connection_pool(&db_url);
    db::init(db_url.as_str());

    let metrics = Arc::new(Metrics::new());
    let prometheus = metrics.prometheus();

    let app_metrics = metrics.clone();
    let app_config = config.clone();
    let app_db_connection_pool = db_connection_pool.clone();
    let app_factory = move || {
        App::new()
            .app_data(web::PathConfig::default().error_handler(error_handler::path_error_handler))
            .wrap(Cors::permissive())
            .wrap(prometheus.clone())
            .wrap(
                Logger::new(
                    "%a \"%r\" \"Authorization=%{Authorization}i\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
                )
                    // prevent log cluttering with frequent requests
                    .exclude("/health")
                    .exclude("/metrics")
            )
            .app_data(web::Data::new(ServerState {
                db_connection_pool: app_db_connection_pool.clone(),
                metrics: app_metrics.clone(),
                config: app_config.clone(),
            }))
            .configure(init_routes)
    };

    let bind_ip = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_port = env::var("BIND_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    info!("Starting server");
    HttpServer::new(app_factory)
        .bind((bind_ip, bind_port))?
        .run()
        .await
}
