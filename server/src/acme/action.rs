use super::{extend, parser};
use serde_json::{json, Value};
use std::error::Error;
use tokio::process::Command;

pub async fn apply(payload: &Value) -> Result<Value, Box<dyn Error>> {
    match payload.get("action").and_then(Value::as_str) {
        Some("ca-account") => extend::ca_account(),
        Some("dns-provider") => extend::dns_provider(),
        Some(action) => acme_sh(action, payload).await,
        None => Err("Invalid action".into()),
    }
}

async fn acme_sh(action: &str, payload: &Value) -> Result<Value, Box<dyn Error>> {
    let output = acme_sh_builder(action, payload).output().await?;

    if output.status.success() {
        let resp = String::from_utf8_lossy(&output.stdout);
        match action {
            "info" => parser::info(&resp),
            "list" => parser::list(&resp),
            "issue" => parser::issue(&resp),
            _ => Ok(json!({"Stdout": resp})),
        }
    } else {
        let resp = String::from_utf8_lossy(&output.stderr);
        Err(resp.into())
    }
}

fn acme_sh_builder(action: &str, payload: &Value) -> Command {
    let mut acme = Command::new("acme.sh");
    acme.env("NO_TIMESTAMP", "1").arg(format!("--{}", action));

    if let Some(obj) = payload.as_object() {
        if let Some(envobj) = obj.get("env").and_then(Value::as_object) {
            for (k, v) in envobj {
                if let Some(s) = v.as_str() {
                    acme.env(k, s);
                }
            }
        }

        for (key, val) in obj {
            if key == "action" || key == "env" {
                continue;
            }

            match val {
                Value::Array(items) => {
                    for item in items {
                        if let Some(s) = item.as_str() {
                            acme.arg(format!("--{}", key)).arg(s);
                        }
                    }
                }
                Value::String(s) => {
                    acme.arg(format!("--{}", key)).arg(s);
                }
                _ => {}
            }
        }
    }

    tracing::info!("run command {:?}", acme.as_std());
    acme
}
