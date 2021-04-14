use std::env;

/// Gets the username based off of if the USER environment variable is set
pub fn get_username() -> String {
    if cfg!(target_os = "windows") {
        return env::var("USERNAME").unwrap_or(String::from(""));
    } else {
        return match env::var("USER") {
            Err(_) => {
                match env::var("LOGNAME") {
                    Err(_) => String::from(""),
                    Ok(username) => username
                }
            },
            Ok(username) => username
        };
    }
}

/// Gets the command line arguments of the current process
pub fn get_commandline_args() -> String {
    let args: Vec<String> = env::args().collect();
    let mut process_cmd_line: String = String::from("");
    for item in args.iter() {
        process_cmd_line += &item;
        process_cmd_line.push_str(" ");
    }
    return process_cmd_line;
}