#[macro_use]
extern crate clap;
extern crate base64;

mod ops;
//use serde::{Deserialize, Serialize};
use serde_json::{Value};
use clap::App;
use base64::{decode};
use std::vec;

// #[derive(Serialize, Deserialize)]
// struct NetworkTask{
//     typ: String,
//     host: String,
//     port: u16,
//     protocol: String,
//     data: String,
//     b64_encoded: bool
// }

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(commands) = matches.value_of("command_file") {
        if let Ok(json_data) = ops::fileops::read_file(commands, 0, 0) {
            let durp: Value = serde_json::from_str(&json_data).unwrap();
            for obj in durp.as_array() {
                for task in obj {
                    if task["type"] == "file" {
                        if task["args"]["command"] == "create" {
                            ops::fileops::create_file(task["file_path"].as_str().unwrap(), task["args"]["data"].as_str().unwrap().as_bytes()).unwrap();
                        } else if task["args"]["command"] == "write" {
                            ops::fileops::write_file(task["file_path"].as_str().unwrap(),
                                                     task["args"]["data"].as_str().unwrap().as_bytes(),
                                                     task["args"]["offset"].as_str().unwrap().parse().unwrap()).unwrap();
                        } else if task["args"]["command"] == "read" {
                            ops::fileops::read_file(task["file_path"].as_str().unwrap(),
                                                    task["args"]["offset"].as_str().unwrap().parse().unwrap(),
                                                    task["args"]["num_bytes"].as_str().unwrap().parse().unwrap()).unwrap();
                        } else if task["args"]["command"] == "delete" {
                            ops::fileops::delete(task["file_path"].as_str().unwrap()).unwrap();
                        } else {
                            panic!("Invalid file command. Valid commands are 'create', 'write', 'read', 'delete'");
                        }
                    } else if task["type"] == "network" {
                        ops::network::send_data(task["args"]["host"].as_str().unwrap(),
                                                task["args"]["port"].as_u64().unwrap() as u16,
                                                task["args"]["protocol"].as_str().unwrap(),
                                                task["args"]["data"].as_str().unwrap().as_bytes());
                    } else if task["type"] == "process" {
                        //let args = task["args"]["args"].as_array().unwrap();
                        //ops::process::exec_file(task["args"]["file_path"].as_str().unwrap(), &args);
                    } else {
                        panic!("Invalid task type. Availabel task type are 'process', 'network', and 'file'");
                    }
                }
            }
        }
    }
    else {
        if let Some(matches) = matches.subcommand_matches("process") {
            if let Some(filepath) = matches.value_of("FILE_PATH") {
                if let Some(arguments) = matches.values_of("arguments") {
                    let args: Vec<&str> = arguments.collect();
                    ops::process::exec_file(filepath, &args);
                } else {
                    ops::process::exec_file(filepath, &vec![]);
                }
            } 
        } else if let Some(matches) = matches.subcommand_matches("network") {
            // Provide defaults for host, port, protocol and data
            let host = matches.value_of("host").unwrap_or("127.0.0.1");

            let port_str = matches.value_of("port").unwrap_or("8080");
            let port: u16 = port_str.parse::<u16>().unwrap_or(8080);

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
        } else if let Some(matches) = matches.subcommand_matches("readfile") {
            let filename = matches.value_of("FILE_PATH").unwrap();

            let offset_str = matches.value_of("offset").unwrap_or("0");
            let offset: u64 = offset_str.parse().unwrap_or(0);

            let num_bytes_str = matches.value_of("num_bytes").unwrap_or("0");
            let num_bytes: usize = num_bytes_str.parse().unwrap_or(0);

            if let Ok(data) = ops::fileops::read_file(filename, offset, num_bytes) {
                println!("{}", data);
            }
        } else if let Some(matches) = matches.subcommand_matches("deletefile") {
            let filename = matches.value_of("FILE_PATH").unwrap();
            ops::fileops::delete(filename).expect("Failed to delete file at");
        } else if let Some(matches) = matches.subcommand_matches("createfile") {
            let filename = matches.value_of("FILE_PATH").unwrap();

            let data: &str = matches.value_of("data").unwrap();
            if matches.is_present("base64_encoded") {
                println!("Its encoded");
                let decoded_data = match decode(&data) {
                    Ok(decoded_data) => decoded_data,
                    Err(_) => panic!("Failed to base64 decode data")
                };
                // TODO: Is there a better way to convert?
                let decoded_data_bytes: &[u8] = &decoded_data;
                ops::fileops::create_file(filename, decoded_data_bytes).expect("Failed to create file");
            } else {
                ops::fileops::create_file(filename, data.as_bytes()).expect("Failed to create file");
            }
        } else if let Some(matches) = matches.subcommand_matches("writefile") {
            let filename = matches.value_of("FILE_PATH").unwrap();

            let offset_str = matches.value_of("offset").unwrap_or("0");
            let offset: u64 = offset_str.parse().unwrap_or(0);

            let data: &str = matches.value_of("data").unwrap();
            println!("Writing to file {} at offset {} data: {}", filename, offset, data);
            if matches.is_present("base64_encoded") {
                println!("Its encoded");
                let decoded_data = match decode(&data) {
                    Ok(decoded_data) => decoded_data,
                    Err(_) => panic!("Failed to base64 decode data")
                };
                // TODO: Is there a better way to convert?
                let decoded_data_bytes: &[u8] = &decoded_data;
                match ops::fileops::write_file(filename, decoded_data_bytes, offset) {
                    Err(e) => println!("Failed to write file: {}", e),
                    Ok(_) => ()
                };
            } else {
                match ops::fileops::write_file(filename, data.as_bytes(), offset) {
                    Err(e) => println!("Failed to write file: {}", e),
                    Ok(_) => ()
                };
            }
        }
    }
}
