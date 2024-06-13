use crate::acme;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json;
use std::{fmt, fs};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub async fn init() {
    let addr = "0.0.0.0:7600";
    let listener = TcpListener::bind(addr).await.unwrap();

    let not_found = error_404.into_service();
    let serve_dir = ServeDir::new("public").not_found_service(not_found);

    let service = Router::new()
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
        .route("/acme", get(acme_handler));

    tracing::info!("listening on {}", addr);
    axum::serve(listener, service).await.unwrap()
}

async fn error_404() -> (StatusCode, String) {
    let content = match fs::read_to_string("public/404.html") {
        Ok(html) => html,
        Err(_) => String::from("404 Not Found"),
    };

    (StatusCode::NOT_FOUND, content)
}

async fn acme_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket: WebSocket| async move {
        while let Some(Ok(recv)) = socket.recv().await {
            let response = match recv {
                Message::Text(text) => acme_app_route(&text).await,
                _ => ws_error_message(1001, &"Not Supported"),
            };
            if let Err(err) = socket.send(response).await {
                tracing::warn!("{}", err);
                break;
            }
        }
    })
}

async fn acme_app_route(text: &str) -> Message {
    match serde_json::from_str(text) {
        Ok(payload) => match acme::app::route(&payload).await {
            Ok(data) => match serde_json::to_string(&data) {
                Ok(body) => Message::Text(body.into()),
                Err(err) => ws_error_message(1004, &err),
            },
            Err(err) => ws_error_message(1003, &err),
        },
        Err(err) => ws_error_message(1002, &err),
    }
}

fn ws_error_message(code: u16, data: &dyn fmt::Display) -> Message {
    let body = format!(r#"{{"code":{},"message":"{}"}}"#, code, data);
    Message::Text(body.into())
}
