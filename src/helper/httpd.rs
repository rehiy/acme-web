use crate::acme;
use axum::{handler::HandlerWithoutStateExt, http::StatusCode};
use axum::{routing::get, Router};
use std::fs::read_to_string;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub async fn listen() {
    let addr = "0.0.0.0:3000";
    let router = handler().layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", addr);
    axum::serve(listener, router).await.unwrap();
}

fn handler() -> Router {
    let not_found = handler_404.into_service();
    let serve_dir = ServeDir::new("public").not_found_service(not_found);

    Router::new()
        .fallback_service(serve_dir)
        .route("/acme/list", get(acme::list::handler))
}

async fn handler_404() -> (StatusCode, String) {
    let content = match read_to_string("public/404.html") {
        Ok(html) => html,
        Err(_) => String::from("404 Not Found"),
    };

    (StatusCode::NOT_FOUND, content)
}
