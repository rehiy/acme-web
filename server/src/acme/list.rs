use serde_json::{json, Value};
use tokio::process::Command;

pub async fn handler() -> Result<Value, String> {
    let mut acme = Command::new("acme.sh");

    match acme.arg("--list").output().await {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                parse_acme_list(&stdout)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(stderr.to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn parse_acme_list(output: &str) -> Result<Value, String> {
    let mut items = Vec::new();
    let mut lines = output.lines();

    lines.next();
    for line in lines {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 6 {
            return Err(format!("Invalid line format: {}", line));
        }
        let item = json!({
            "MainDomain": parts[0],
            "KeyLength": parts[1],
            "SANDomains": parts[2],
            "CA": parts[3],
            "Created": parts[4],
            "Renew": parts[5],
        });
        items.push(item);
    }

    Ok(serde_json::Value::Array(items))
}
