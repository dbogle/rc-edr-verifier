
#[macro_use]
extern crate clap;
extern crate base64;

mod ops;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use clap::App;
use base64::{decode};
use std::vec;
use log::{error};
use log4rs;


#[derive(Serialize, Deserialize)]
struct NetworkTask {
    host: String,
    port: u16,
    protocol: String,
    data: String,
    b64_encoded: bool
}

/*
Network connection and data transmission
Timestamp of activity
Username that started the process that initiated the network activity
Destination address and port
Source address and port
Amount of data sent
Protocol of data sent
Process name
Process command line
Process ID
*/



#[derive(Serialize, Deserialize)]
struct ProcessTask {
    filepath: String,
    args: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct CreateFileTask {
    filepath: String,
    command: String,
    data: String,
    b64_encoded: bool
}

#[derive(Serialize, Deserialize)]
struct ReadFileTask {
    filepath: String,
    command: String,
    offset: u64,
    num_bytes: usize
}

#[derive(Serialize, Deserialize)]
struct WriteFileTask {
    filepath: String,
    command: String,
    offset: u64,
    data: String,
    b64_encoded: bool
}

#[derive(Serialize, Deserialize)]
struct DeleteFileTask {
    filepath: String,
    command: String,
}


fn main() {
    let yaml = load_yaml!("cli.yml");
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    let matches = App::from_yaml(yaml).get_matches();

    if let Some(commands) = matches.value_of("command_file") {
        if let Ok(json_data) = ops::fileops::read_file(commands, 0, 0) {
            let command_data: Value = serde_json::from_str(&json_data).unwrap();
            for tasks in command_data.as_array() {
                for task in tasks {
                    let args = task["args"].clone();
                    if task["type"] == "file" {
                        if args["command"] == "create" {
                            match serde_json::from_value::<CreateFileTask>(args) {
                                Ok(c) => {
                                    if ops::fileops::create_file(&c.filepath, c.data.as_bytes()).is_err() {
                                        error!("Failed to create file");
                                    }
                                },
                                Err(_) => {
                                    error!("Failed to deserialize json into CreateFileTask");
                                    continue;
                                }
                            };
                        } else if args["command"] == "write" {
                            let w: WriteFileTask = match serde_json::from_value(args) {
                                Ok(w) => w,
                                Err(_) => continue
                            };
                            if ops::fileops::write_file(&w.filepath, w.data.as_bytes(), w.offset).is_err() {
                                error!("Failed to write file");
                            }
                        } else if args["command"] == "read" {
                            let r: ReadFileTask = match serde_json::from_value(args) {
                                Ok(r) => r,
                                Err(_) => continue
                            };
                            match ops::fileops::read_file(&r.filepath, r.offset, r.num_bytes) {
                                Ok(res) => println!("{}", res),
                                Err(_) => continue
                            };
                        } else if args["command"] == "delete" {
                            let d: DeleteFileTask = match serde_json::from_value(args) {
                                Ok(d) => d,
                                Err(_) => continue
                            };
                            if ops::fileops::delete(&d.filepath).is_err() {
                                error!("Failed to delete file at '{}'", d.filepath);
                            }
                        } else {
                            panic!("Invalid file command. Valid commands are 'create', 'write', 'read', 'delete'");
                        }
                    } else if task["type"] == "network" {
                        let n: NetworkTask = match serde_json::from_value(args) {
                            Ok(n) => n,
                            Err(_) => continue
                        };
                        if ops::network::send_data(&n.host, n.port, n.protocol, n.data.as_bytes()).is_err() {
                            error!("Failed to send bytes on the network");
                        }
                    } else if task["type"] == "process" {
                        let p: ProcessTask = match serde_json::from_value(args) {
                            Ok(p) => p,
                            Err(_) => continue
                        };
                        ops::process::exec_file2(&p.filepath, p.args);
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
                    let args: Vec<_> = arguments.collect();
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
                if ops::network::send_data(&host.to_string(), port, protocol.to_string(), decoded_data_bytes).is_err() {
                    error!("Failed to send bytes on the network");
                }
            } else {
                if ops::network::send_data(&host.to_string(), port, protocol.to_string(), data.as_bytes()).is_err() {
                    error!("Failed to send bytes on the network");
                }
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
