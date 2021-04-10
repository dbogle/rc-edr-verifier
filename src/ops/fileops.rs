//use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
//use std::os::unix;
use std::path::Path;
use std::fs;
use std::io::SeekFrom;

pub fn read_file(filename: &str, offset: u64, num_bytes: usize) -> io::Result<String> {
    let path = Path::new(filename);
    let mut f = File::open(path)?;
    let mut s = String::new();
    if offset > 0 {
        f.seek(SeekFrom::Start(offset))?;
    }
    if num_bytes > 0 {
        let mut limited = f.take(num_bytes as u64);
        match limited.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    } else {
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

}

pub fn create_file(filepath: &str, data: &[u8]) -> io::Result<()> {
    let mut f = File::create(filepath)?;
    return f.write_all(data);
}

pub fn write_file(filepath: &str, data: &[u8], offset: u64) -> io::Result<()> {
    let path = Path::new(filepath);
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    println!("Got past open");
    f.seek(SeekFrom::Start(offset))?;
    return f.write_all(data);
}

pub fn delete(filename: &str) -> io::Result<bool> {
    let f = File::open(filename)?;
    let meta = f.metadata()?;
    if meta.is_dir() {
        match fs::remove_dir_all(filename) {
            Err(why) => {
                println!("remove_dir_all failed: {:?}", why.kind());
                Err(why)
            },
            Ok(_) => Ok(true)
        }
    } else {
        match fs::remove_file(filename) {
            Err(why) => {
                println!("remove_file failed: {:?}", why.kind());
                Err(why)
            },
            Ok(_) => Ok(true)
        }
    }
}