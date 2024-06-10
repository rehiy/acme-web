use axum::http::StatusCode;
use axum::response::{IntoResponse, Json as JsonResponse};
use std::collections::HashMap;
use tokio::process::Command;

pub async fn handler() -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {
    let mut acme = Command::new("acme.sh");

    match acme.arg("--info").output().await {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                match parse_acme_list(&stdout) {
                    Ok(json) => Ok((StatusCode::OK, JsonResponse(json))),
                    Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, JsonResponse(err))),
                }
            } else {
                let err = String::from_utf8_lossy(&output.stderr).to_string();
                Err((StatusCode::INTERNAL_SERVER_ERROR, JsonResponse(err)))
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(e.to_string()),
        )),
    }
}

fn parse_acme_list(output: &str) -> Result<HashMap<String, String>, String> {
    let mut items = HashMap::new();
    let mut lines = output.lines();

    lines.next();
    for mut line in lines {
        line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim().trim_matches('\'');
            items.insert(key.to_string(), value.to_string());
        }
    }

    Ok(items)
}
