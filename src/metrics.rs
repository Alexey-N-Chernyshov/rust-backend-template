use crate::db::Pool;
use actix_web::rt::time;
use actix_web_prom::PrometheusMetrics;
use actix_web_prom::PrometheusMetricsBuilder;
use log::error;
use prometheus::IntGauge;
use std::sync::Arc;
use std::time::Duration;

/// Metrics update interval in seconds.
pub const METRICS_UPDATE_INTERVAL: u64 = 60;

/// Provides additional metrics.
pub struct Metrics {
    // TODO set up metrics
    pub my_project_metric: IntGauge,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            // TODO set up metrics
            my_project_metric: IntGauge::new("my_project_metric", "my_project_metric description")
                .unwrap(),
        }
    }

    pub fn prometheus(&self) -> PrometheusMetrics {
        // TODO add metrics to prometheus
        PrometheusMetricsBuilder::new("api")
            .endpoint("/metrics")
            .build()
            .unwrap()
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a periodic task to update business metrics.
pub fn spawn_metrics_tasks(metrics: Arc<Metrics>, db_connection_pool: Pool) {
    actix_rt::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(METRICS_UPDATE_INTERVAL));
        loop {
            interval.tick().await;

            match db_connection_pool.get() {
                Ok(mut _db_connection) => {
                    // TODO update metrics
                    metrics.my_project_metric.set(42);
                }
                Err(err) => {
                    error!("Cannot connect to DB: {err}");
                }
            }
        }
    });
}
