extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

use std::process::Command;
use std::{fs, str};

const YmlPath: &'static str = "rsync.config.yml";

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;
    let hello_str = str::from_utf8(&hello)?;

    let yml_str = fs::read_to_string(YmlPath)
        .expect(&format!("should exist {}", YmlPath));

    let docs = YamlLoader::load_from_str(&yml_str)?;
    dbg!(docs);

    Ok(())
}
