use crate::acme;
use axum::{extract::Path, routing::post, Router};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub async fn init() {
    let address = "0.0.0.0:7600";
    let listener = TcpListener::bind(address).await.unwrap();

    let not_found = ServeFile::new("public/404.html");
    let serve_dir = ServeDir::new("public").fallback(not_found);

    let service = Router::new()
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .route("/acme/:act", post(acme_handler));

    tracing::info!("listening on {}", address);
    axum::serve(listener, service).await.unwrap()
}

async fn acme_handler(act: Path<String>, body: axum::Json<Value>) -> axum::Json<Value> {
    match acme::action::apply(&act, &body).await {
        Ok(data) => axum::Json(data),
        Err(err) => axum::Json(json!({
            "Message": err.to_string(),
            "Type": "error",
        })),
    }
}
