use serde_json::{json, Value};
use tokio::process::Command;

pub async fn handler() -> Result<Value, String> {
    let mut acme = Command::new("acme.sh");

    match acme.arg("--info").output().await {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                parse_acme_info(&stdout)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(stderr.to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn parse_acme_info(output: &str) -> Result<Value, String> {
    let mut items = json!({});
    let mut lines = output.lines();

    lines.next();
    for mut line in lines {
        line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let val = parts[1].trim().trim_matches('\'').to_string();
            items[key] = serde_json::Value::String(val);
        }
    }

    Ok(items)
}
