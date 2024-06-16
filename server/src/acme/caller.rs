use super::stdout;
use serde_json::{json, Value};
use tokio::process::Command;

pub async fn apply(payload: &Value) -> Result<Value, String> {
    let mut acme: Command = Command::new("acme.sh");
    let args = json_to_args(payload).replace("--action ", "--");
    let action = payload.get("action").and_then(|c| c.as_str()).unwrap_or("");

    match acme.arg(args).output().await {
        Ok(output) => {
            if output.status.success() {
                let body = String::from_utf8_lossy(&output.stdout);
                match action {
                    "info" => stdout::info(&body),
                    "list" => stdout::list(&body),
                    "issue" => stdout::issue(&body),
                    _ => Ok(json!({"action": action, "stdout": body})),
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
