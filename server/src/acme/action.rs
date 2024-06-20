use super::{extend, parser};
use serde_json::{json, Value};
use std::error::Error;
use tokio::process::Command;

pub async fn apply(action: &str, payload: &Value) -> Result<Value, Box<dyn Error>> {
    match action {
        "ca-account" => extend::ca_account(),
        "dns-provider" => extend::dns_provider(),
        _ => acme_sh(action, payload).await,
    }
}

// 调用系统 acme.sh 命令

async fn acme_sh(action: &str, payload: &Value) -> Result<Value, Box<dyn Error>> {
    let mut acme = Command::new("acme.sh");
    acme.env("NO_TIMESTAMP", "1").arg(format!("--{}", action));
    // 解析环境变量和参数
    if let Some(obj) = payload.as_object() {
        if let Some(envobj) = obj.get("env").and_then(Value::as_object) {
            for (k, v) in envobj {
                v.as_str().map(|s| acme.env(k, s));
            }
        }
        // 解析子命令参数
        for (key, val) in obj {
            if key == "env" {
                continue;
            }
            match val {
                Value::Array(items) => {
                    for v in items {
                        v.as_str().map(|s| acme.arg(format!("--{}", key)).arg(s));
                    }
                }
                Value::String(s) => {
                    acme.arg(format!("--{}", key)).arg(s);
                }
                _ => {}
            }
        }
    }
    // 执行 acme.sh 命令
    tracing::info!("run command {:?}", acme.as_std());
    let output = acme.output().await?;
    // 解析 acme.sh 命令输出
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
