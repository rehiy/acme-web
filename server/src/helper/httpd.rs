use crate::acme;
use axum::{handler::HandlerWithoutStateExt, http::StatusCode};
use axum::{routing::post, Router};
use std::fs::read_to_string;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub async fn listen() {
    let addr = "0.0.0.0:7600";
    let router = handler().layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", addr);
    axum::serve(listener, router).await.unwrap();
}

async fn error_404() -> (StatusCode, String) {
    let content = match read_to_string("public/404.html") {
        Ok(html) => html,
        Err(_) => String::from("404 Not Found"),
    };

    (StatusCode::NOT_FOUND, content)
}

fn handler() -> Router {
    let not_found = error_404.into_service();
    let serve_dir = ServeDir::new("public").not_found_service(not_found);

    Router::new()
        .fallback_service(serve_dir)
        .route("/acme/info", post(acme::info::handler))
        .route("/acme/list", post(acme::list::handler))
}
