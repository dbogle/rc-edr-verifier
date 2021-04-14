use std::process::Command;
use log::{info};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use crate::ops::util::*;
use std::env;

#[derive(Serialize, Deserialize)]
struct ProcessTaskLogEntry {
    timestamp: String,
    user: String,
    process_name: String,
    process_args: String,
    process_id: u64
}

pub fn exec_file(filepath: &str, args: &[&str]) {
    // TODO: Add logging here
    let _proc = Command::new(filepath)
                    .args(args)
                    .spawn()
                    .expect("Failed to run command");
    let proc_log = json!({
        "timestamp": SystemTime::now(),
        "user": get_username(),
        "process_name": env::args().nth(0),
        "process_args": get_commandline_args(),
        "process_id": std::process::id()
    });
    info!("{}", proc_log);
}

pub fn exec_file2(filepath: &str, args: Vec<String>) {
    // TODO: Add logging here
    println!("Executing: {} {:?}", filepath, args);
    let _proc = Command::new(filepath)
        .args(args)
        .spawn()
        .expect("Failed to run command");
    let proc_log = json!({
        "timestamp": SystemTime::now(),
        "user": get_username(),
        "process_name": env::args().nth(0),
        "process_args": get_commandline_args(),
        "process_id": std::process::id()
    });
    info!("{}", proc_log);
}