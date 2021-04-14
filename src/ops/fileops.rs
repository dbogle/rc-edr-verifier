use std::env;
use std::io;
use std::fs;
use std::path::Path;
use serde_json::json;
use std::io::SeekFrom;
use log::{info, error};
use std::io::prelude::*;
use std::time::SystemTime;
use std::fs::{File, OpenOptions};
use serde::{Deserialize, Serialize};

use crate::ops::util::*;

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

/// Reads a file at the specified path
/// 
/// # Arguments
/// 
/// * `filename` - The path to the file to create
/// * `offset` - Thee offset to start reading from
/// * `num_bytes` - The number of bytes to read
/// 
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

/// Creates a file at the specified filepath
/// 
/// # Arguments
/// 
/// * `filepath` - The path to the file to create
/// * `data` - The data to write to the file
/// 
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

/// Writes data to the given file at the specified offset
/// 
/// # Arguments
/// 
/// * `filepath` - The path to the file to write the data to
/// * `data` - The data to write to the file
/// * `offset` - The offset at which to write the data at. If the offset is larger
///              than the initial file size it will extend the filesize with zeros 
/// 
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

/// Deletes a file or directory at the specified path
/// 
/// # Arguments
/// 
/// * `filename` - The path to the file to create
/// 
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
                error!("remove_dir_all failed: {:?}", why.kind());
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
                error!("remove_file failed: {:?}", why.kind());
                Err(why)
            },
            Ok(_) => {
                info!("{}", delete_log);
                Ok(true)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let filepath = String::from("./durpdurp.txt");
        let data = String::from("Somedatainthefile");
        let mut f = File::create(&filepath).unwrap();
        f.write_all(data.as_bytes()).unwrap();
        let read_data = read_file(&filepath, 0, data.len()).unwrap();
        assert_eq!(read_data, data);
        delete(&filepath).unwrap();
    }
}