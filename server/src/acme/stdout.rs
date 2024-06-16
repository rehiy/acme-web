use serde_json::{json, Value};

pub fn info(output: &str) -> Result<Value, String> {
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

pub fn list(output: &str) -> Result<Value, String> {
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

pub fn issue(output: &str) -> Result<Value, String> {
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
