use axum::http::StatusCode;
use axum::response::{IntoResponse, Json as JsonResponse};
use serde_json::json;
use tokio::process::Command;

pub async fn handler() -> Result<(StatusCode, impl IntoResponse), (StatusCode, impl IntoResponse)> {
    let mut acme = Command::new("acme.sh");

    match acme.arg("--list").output().await {
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

fn parse_acme_list(output: &str) -> Result<Vec<serde_json::Value>, String> {
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

    Ok(items)
}
