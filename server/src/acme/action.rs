use super::{helper, parser};
use serde_json::{json, Value};
use std::error::Error;
use tokio::process::Command;

pub async fn apply(payload: &Value) -> Result<Value, Box<dyn Error>> {
    let act = payload["action"].as_str().unwrap_or("");

    match act {
        "ca-account" => helper::ca_account(),
        _ => acme_sh(act, payload).await,
    }
}

async fn acme_sh(act: &str, payload: &Value) -> Result<Value, Box<dyn Error>> {
    let output = acme_sh_builder(payload).output().await?;

    if output.status.success() {
        let resp = String::from_utf8_lossy(&output.stdout);
        return match act {
            "info" => parser::info(&resp),
            "list" => parser::list(&resp),
            "issue" => parser::issue(&resp),
            _ => Ok(json!({"Stdout": resp})),
        };
    } else {
        let resp = String::from_utf8_lossy(&output.stderr);
        return Err(resp.into());
    }
}

fn acme_sh_builder(payload: &Value) -> Command {
    let mut acme = Command::new("acme.sh");

    if let Some(obj) = payload.as_object() {
        for (key, val) in obj.iter() {
            match val {
                Value::Array(items) => {
                    for item in items {
                        if let Some(v) = item.as_str() {
                            acme.arg(format!("--{}", key)).arg(format!("{}", v));
                        }
                    }
                }
                Value::String(v) => {
                    match key.as_str() {
                        "action" => {
                            acme.arg(format!("--{}", v));
                        }
                        k if k.starts_with("env") => {
                            acme.env(k.replace("env_", ""), v);
                        }
                        _ => {
                            acme.arg(format!("--{}", key)).arg(format!("{}", v));
                        }
                    };
                }
                _ => {
                    acme.arg(format!("--{}", key)).arg(format!("{}", val));
                }
            }
        }
    }

    acme.env("NO_TIMESTAMP", "1");
    tracing::info!("run command {:?}", acme.as_std());

    return acme;
}
