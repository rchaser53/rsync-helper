extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

use std::process::Command;
use std::{fs, str};

const YML_PATH: &'static str = "rsync.config.yml";

type StaticError = Box<dyn std::error::Error + 'static>;
fn main() -> Result<(), StaticError> {
    let config = extract_config(YML_PATH);
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process");
    Ok(())
}

#[derive(Debug)]
struct YmlConfig {
    rsync_path: String,
    ignored: Vec<Yaml>,
    ssh_key_path: String,
}

fn extract_config<T: AsRef<str>>(yml_path: T) -> Result<YmlConfig, StaticError>
where
    T: std::convert::AsRef<std::path::Path> + std::fmt::Display,
{
    let yml_str = fs::read_to_string(&yml_path).expect(&format!("should exist {}", yml_path));

    let docs = YamlLoader::load_from_str(&yml_str)?;
    let doc = &docs[0];

    let rsync_path = doc["rsync_path"].as_str().expect("rsync_path is required");
    let ignored: &Vec<Yaml> = doc["ignored"].as_vec().expect("ignored is required");
    let ssh_key_path = doc["ssh_key_path"]
        .as_str()
        .expect("ssh_key_path is required");

    Ok(YmlConfig {
        rsync_path: rsync_path.into(),
        ignored: ignored.to_vec(),
        ssh_key_path: ssh_key_path.into(),
    })
}
