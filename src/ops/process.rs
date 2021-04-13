use std::process::Command;
use std::time::SystemTime;
use log::{info};

// struct ProcessTaskLogEntry {
//     timestamp: String,
//     user: String,
//     process_name: String,
//     process_args: String,
//     process_id: u64
// }

pub fn exec_file(filepath: &str, args: &[&str]) {
    // TODO: Add logging here
    let proc = Command::new(filepath)
                    .args(args)
                    .spawn()
                    .expect("Failed to run command");
    info!("{:#?} | {}", SystemTime::now(), proc.id());
}

pub fn exec_file2(filepath: &str, args: Vec<String>) {
    // TODO: Add logging here
    println!("Executing: {} {:?}", filepath, args);
    let proc = Command::new(filepath)
        .args(args)
        .spawn()
        .expect("Failed to run command");
}