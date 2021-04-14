use std::process::Command;
use log::{info, error};
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
    match Command::new(filepath).args(args).spawn() {
        Ok(_) => {
            let proc_log = json!({
                "timestamp": SystemTime::now(),
                "user": get_username(),
                "process_name": env::args().nth(0),
                "process_args": get_commandline_args(),
                "process_id": std::process::id()
            });
            info!("{}", proc_log);
        },
        Err(err) => error!("Failed to execute process: {:?}", err.kind())
    };
}

pub fn exec_file2(filepath: &str, args: Vec<String>) {
    match Command::new(filepath).args(args).spawn() {
        Ok(_) => {
            let proc_log = json!({
                "timestamp": SystemTime::now(),
                "user": get_username(),
                "process_name": env::args().nth(0),
                "process_args": get_commandline_args(),
                "process_id": std::process::id()
            });
            info!("{}", proc_log);
        },
        Err(err) => error!("Failed to execute process: {:?}", err.kind())
    }
}