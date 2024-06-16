use super::stdout;
use serde_json::{json, Value};
use tokio::process::Command;

pub async fn apply(payload: &Value) -> Result<Value, String> {
    let mut cmd: Command = Command::new("acme.sh");
    let args = json_to_args(payload).replace("--command ", "--");

    match cmd.arg(args).output().await {
        Ok(output) => {
            if output.status.success() {
                let body = String::from_utf8_lossy(&output.stdout);
                match payload.get("command").and_then(|c| c.as_str()) {
                    Some("ping") => Ok(json!({"command": "pong"})),
                    Some("info") => stdout::info(&body),
                    Some("list") => stdout::list(&body),
                    Some("issue") => stdout::issue(&body),
                    _ => Err("Not Found Route".to_string()),
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(stderr.to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

fn json_to_args(json_data: &Value) -> String {
    if let Some(obj) = json_data.as_object() {
        obj.iter()
            .flat_map(|(key, val)| match val {
                Value::String(s) => vec![format!("--{} {}", key, s)],
                Value::Array(arr) => arr
                    .iter()
                    .filter_map(|elem| {
                        if let Value::String(s) = elem {
                            Some(format!("--{} {}", key, s))
                        } else {
                            None
                        }
                    })
                    .collect(),
                _ => vec![],
            })
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        "--help".to_string()
    }
}
