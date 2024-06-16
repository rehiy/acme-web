use crate::acme;
use axum::{handler::HandlerWithoutStateExt, http::StatusCode, routing::get, Router};
use std::fs;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub async fn init() {
    let address = "0.0.0.0:7600";
    let listener = TcpListener::bind(address).await.unwrap();

    let not_found = error_404.into_service();
    let serve_dir = ServeDir::new("public").not_found_service(not_found);

    let service = Router::new()
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .route("/acme", get(acme::websocket::handler));

    tracing::info!("listening on {}", address);
    axum::serve(listener, service).await.unwrap()
}

async fn error_404() -> (StatusCode, String) {
    let content = match fs::read_to_string("public/404.html") {
        Ok(html) => html,
        Err(_) => String::from("404 Not Found"),
    };

    (StatusCode::NOT_FOUND, content)
}
