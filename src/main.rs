use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;

#[tokio::main]
async fn main() {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/health", get(|| async { "OK"}))
        .layer(prometheus_layer);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

