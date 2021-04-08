#[macro_use]
extern crate clap;
extern crate base64;

mod ops;
use clap::App;
use base64::{decode};
use std::vec;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(commands) = matches.value_of("command_file") {
        println!("Command file to parse: {}", commands)
        // Run the various commands in the json file
        // I would like something like
        /*
        for (item in jsonarray) {
            if item["type"] == "process" {
                ops::process::exec_file(args);
            } else if item["type"] == "network" {
                ops::network::send_data(args);
            } else if item["type"] == "file" {
                if item["task_args"]["command"] == "create" {
                    ops::fileops::create(args);
                } else if item["task_args"]["command"] == "read" {
                    ops::fileops::read(args);
                } else if item["task_args"]["command"] == "write" {
                    ops::fileops::write(args);
                } else if item["task_args"]["command"] == "delete" {
                    ops::fileops::delete(args);
                } else {
                    println!("Invalid command for file");
                }
            }
        }
        */
    }
    else {
        if let Some(matches) = matches.subcommand_matches("process") {
            if let Some(filepath) = matches.value_of("FILE_PATH") {
                if let Some(arguments) = matches.values_of("arguments") {
                    ops::process::exec_file(filepath, arguments.collect());
                } else {
                    ops::process::exec_file(filepath, vec![]);
                }
            } 
        }
        else if let Some(matches) = matches.subcommand_matches("network") {
            // Provide defaults for host, port, protocol and data
            let host = matches.value_of("host").unwrap_or("127.0.0.1");
            let port_str = matches.value_of("port").unwrap_or("8080");
            let port:u16 = port_str.parse::<u16>().unwrap_or(8080);
            let protocol = matches.value_of("protocol").unwrap_or("tcp");
            let data = matches.value_of("data").unwrap_or("Hello world\n");
            if matches.is_present("base64_encoded") {
                let decoded_data = match decode(&data) {
                    Ok(decoded_data) => decoded_data,
                    Err(_) => panic!("Failed to base64 decode data")
                };
                // TODO: Is there a better way to convert?
                let decoded_data_bytes: &[u8] = &decoded_data;
                ops::network::send_data(host, port, protocol, decoded_data_bytes);
            } else {
                ops::network::send_data(host, port, protocol, data.as_bytes());
            }
        }
        else if let Some(matches) = matches.subcommand_matches("file") {
            if matches.is_present("FILE_PATH") {
                //if let Some(filepath) = matches.value_of
            }
        }
    }
}
