//use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
//use std::os::unix;
use std::path::Path;
use std::fs;
use std::io::SeekFrom;
use log::{info};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use crate::ops::util::*;
use std::env;

#[derive(Serialize, Deserialize)]
struct FileTaskLogEntry {
    timestamp: String,
    filepath: String,
    activity: String,
    user: String,
    process_name: String,
    process_args: String,
    process_id: u64
}

pub fn read_file(filename: &str, offset: u64, num_bytes: usize) -> io::Result<String> {
    let path = Path::new(filename);
    let mut f = File::open(path)?;
    let mut s = String::new();
    if offset > 0 {
        f.seek(SeekFrom::Start(offset))?;
    }
    let read_log = json!({
        "timestamp": SystemTime::now(),
        "filepath": filename,
        "activity": "read",
        "user": get_username(),
        "process_name": env::args().nth(0),
        "process_args": get_commandline_args(),
        "process_id": std::process::id()
    });
    if num_bytes > 0 {
        let mut limited = f.take(num_bytes as u64);
        return match limited.read_to_string(&mut s) {
            Ok(_) => {
                info!("{}", read_log);
                return Ok(s)
            },
            Err(e) => Err(e),
        }
    } else {
        return match f.read_to_string(&mut s) {
            Ok(_) => {
                info!("{}", read_log);
                return Ok(s)
            },
            Err(e) => Err(e),
        }
    }
}

pub fn create_file(filepath: &str, data: &[u8]) -> io::Result<()> {
    let mut f = File::create(filepath)?;
    let result = f.write_all(data);
    let create_log = json!({
        "timestamp": SystemTime::now(),
        "filepath": filepath,
        "activity": "create",
        "user": get_username(),
        "process_name": env::args().nth(0),
        "process_args": get_commandline_args(),
        "process_id": std::process::id()
    });
    if result.is_ok() {
        info!("{}", create_log);
    }
    return result;
}

pub fn write_file(filepath: &str, data: &[u8], offset: u64) -> io::Result<()> {
    let path = Path::new(filepath);
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    f.seek(SeekFrom::Start(offset))?;
    let result = f.write_all(data);
    let write_log = json!({
        "timestamp": SystemTime::now(),
        "filepath": filepath,
        "activity": "write",
        "user": get_username(),
        "process_name": env::args().nth(0),
        "process_args": get_commandline_args(),
        "process_id": std::process::id()
    });
    if result.is_ok() {
        info!("{}", write_log);
    }
    return result;
}

pub fn delete(filename: &str) -> io::Result<bool> {
    let f = File::open(filename)?;
    let meta = f.metadata()?;
    let delete_log = json!({
        "timestamp": SystemTime::now(),
        "filepath": filename,
        "activity": "delete",
        "user": get_username(),
        "process_name": env::args().nth(0),
        "process_args": get_commandline_args(),
        "process_id": std::process::id()
    });
    if meta.is_dir() {
        match fs::remove_dir_all(filename) {
            Err(why) => {
                println!("remove_dir_all failed: {:?}", why.kind());
                Err(why)
            },
            Ok(_) => {
                info!("{}", delete_log);
                Ok(true)
            }
        }
    } else {
        match fs::remove_file(filename) {
            Err(why) => {
                println!("remove_file failed: {:?}", why.kind());
                Err(why)
            },
            Ok(_) => {
                info!("{}", delete_log);
                Ok(true)
            }
        }
    }
}