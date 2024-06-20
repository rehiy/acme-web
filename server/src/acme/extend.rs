use glob::glob;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::{error::Error, fs, path::PathBuf};

const AMCE_HOME: &str = "/root/.acme.sh";

// 读取所有 ca.conf 中的账号信息

pub fn ca_account() -> Result<Value, Box<dyn Error>> {
    let mut items = Vec::new();

    let pattern = format!("{}/ca/**/ca.conf", AMCE_HOME);
    for entry in glob(&pattern)? {
        let entry_path = entry?;
        items.push(parse_ca_conf(&entry_path)?);
    }

    Ok(json!({"Payload": items}))
}

fn parse_ca_conf(pb: &PathBuf) -> Result<Value, Box<dyn Error>> {
    let re = Regex::new(r"(?m)^(\w+)='(.+)'$")?;
    let content = fs::read_to_string(pb)?;

    let mut items = HashMap::new();
    for cap in re.captures_iter(&content) {
        items.insert(cap[1].to_string(), cap[2].to_string());
    }

    Ok(json!(items))
}

// 读取所有 dns_*.sh 中的环境变量信息

pub fn dns_provider() -> Result<Value, Box<dyn Error>> {
    let mut items = json!({});

    let pattern = format!("{}/dnsapi/dns_*.sh", AMCE_HOME);
    for entry in glob(&pattern)? {
        let entry_path = entry?;
        if let Some(file_name) = entry_path.file_stem() {
            if let Some(fk) = file_name.to_str() {
                items[fk] = parse_dns_env(&entry_path)?;
            }
        }
    }

    Ok(json!({"Payload": items}))
}

fn parse_dns_env(pb: &PathBuf) -> Result<Value, Box<dyn Error>> {
    let re = Regex::new(r"_readaccountconf_mutable\s+(\w+)").unwrap();
    let content = fs::read_to_string(pb)?;

    let mut items = HashSet::new();
    for cap in re.captures_iter(&content) {
        if let Some(var_name) = cap.get(1) {
            items.insert(var_name.as_str().to_string());
        }
    }

    Ok(json!(items))
}
