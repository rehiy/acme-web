use glob::glob;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::{error::Error, fs, path::PathBuf};

pub fn ca_account() -> Result<Value, Box<dyn Error>> {
    let mut items = Vec::new();

    let fp = "/root/.acme.sh/ca/**/ca.conf";
    for entry in glob(&fp)? {
        items.push(parse_ca_conf(&entry?)?);
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
