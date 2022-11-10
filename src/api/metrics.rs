use actix_web_prom::PrometheusMetricsBuilder;

pub fn metrics_middleware() -> actix_web_prom::PrometheusMetrics {
    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap()
}
