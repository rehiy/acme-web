use serde_json;
use serde_json::{json, Value};

pub fn route(payload: &Value) -> Result<Value, String> {
    let command = &payload["command"];

    if command == "ping" {
        let response = json!({
            "command": "pong".to_string(),
        });
        return Ok(response);
    }

    Err("Not Found Route".to_string())
}
