extern crate termion;
// extern crate notify;
extern crate yaml_rust;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use yaml_rust::{Yaml, YamlLoader};

// use notify::{RecommendedWatcher, Watcher, RecursiveMode};
// use std::sync::mpsc::channel;
// use std::time::Duration;

use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::{fs, str};

const YML_PATH: &'static str = "rsync.config.yml";

type StaticError = Box<dyn std::error::Error + 'static>;
fn main() -> Result<(), StaticError> {
    let config = extract_config(YML_PATH);
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute process");

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
        stdout.flush().unwrap();
    }

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

// let (tx, rx) = channel();
// let mut watcher: RecommendedWatcher = r#try!(Watcher::new(tx, Duration::from_secs(2)));
// r#try!(watcher.watch("./.git/index", RecursiveMode::Recursive));
// loop {
//     match rx.recv() {
//         Ok(event) => println!("{:?}", event),
//         Err(e) => println!("watch error: {:?}", e),
//     }
// }