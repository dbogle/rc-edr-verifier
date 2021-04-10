use std::process::Command;

pub fn exec_file(filepath: &str, args: &[&str]) {
    // TODO: Add logging here
    println!("Executing: {} {:?}", filepath, args);
    Command::new(filepath)
        .args(args)
        .spawn()
        .expect("Failed to run command");
}