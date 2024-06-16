use super::caller;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use serde_json;
use std::fmt;

pub async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket: WebSocket| async move {
        while let Some(Ok(recv)) = socket.recv().await {
            let response = match recv {
                Message::Text(text) => sh_transfer(&text).await,
                _ => error_message(1001, &"Not Supported"),
            };
            if let Err(err) = socket.send(response).await {
                tracing::warn!("{}", err);
                break;
            }
        }
    })
}

async fn sh_transfer(text: &str) -> Message {
    match serde_json::from_str(text) {
        Ok(payload) => match caller::apply(&payload).await {
            Ok(data) => match serde_json::to_string(&data) {
                Ok(body) => Message::Text(body.into()),
                Err(err) => error_message(1004, &err),
            },
            Err(err) => error_message(1003, &err),
        },
        Err(err) => error_message(1002, &err),
    }
}

fn error_message(code: u16, data: &dyn fmt::Display) -> Message {
    tracing::error!("Websocket Error, Code={}, Message={}", code, data);
    let body = format!(r#"{{"code":{},"message":"{}"}}"#, code, data);
    Message::Text(body.into())
}
