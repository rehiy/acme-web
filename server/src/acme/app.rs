use super::{info, list};
use serde_json::{json, Value};

pub async fn route(payload: &Value) -> Result<Value, String> {
    match payload.get("command").and_then(|c| c.as_str()) {
        Some("ping") => Ok(json!({"command": "pong"})),
        Some("info") => info::handler().await,
        Some("list") => list::handler().await,
        _ => Err("Not Found Route".to_string()),
    }
}
