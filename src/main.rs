use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use log::info;
use my_project_name::config::ServerConfig;
use my_project_name::db::create_connection_pool;
use my_project_name::metrics::Metrics;
use my_project_name::routes::init_routes;
use my_project_name::server_state::ServerState;
use my_project_name::{db, error_handler};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;
use std::io::{Error, ErrorKind};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Start my-project-name {}", env!("GIT_PRETTY_VERSION"));

    let config = ServerConfig::new_from_envs();

    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    let db_connection_pool = create_connection_pool(&db_url);
    db::init(db_url.as_str());

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    let metrics = Arc::new(Metrics::new());
    // TODO add metrics to prometheus

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

    let ssl_support = env::var("SSL_ENABLED")
        .unwrap_or_else(|_| "false".to_string())
        .to_ascii_lowercase();
    match ssl_support.as_str() {
        "true" => {
            info!("Starting with SSL_ENABLED");
            let ssl_key_path =
                env::var("SSL_KEY_PATH").unwrap_or_else(|_| "ssl/key.pem".to_string());
            let ssl_cert_path =
                env::var("SSL_CERT_PATH").unwrap_or_else(|_| "ssl/cert.pem".to_string());
            let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
            ssl_builder
                .set_private_key_file(ssl_key_path, SslFiletype::PEM)
                .unwrap();
            ssl_builder
                .set_certificate_chain_file(ssl_cert_path)
                .unwrap();

            HttpServer::new(app_factory)
                .bind_openssl((bind_ip, bind_port), ssl_builder)?
                .run()
                .await
        }
        "false" => {
            info!("Starting without TLS.");
            HttpServer::new(app_factory)
                .bind((bind_ip, bind_port))?
                .run()
                .await
        }
        _ => Err(Error::new(
            ErrorKind::Other,
            "Wrong SSL_ENABLED environment variable value, must be `true` or `false`!",
        )),
    }
}
