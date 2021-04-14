use crate::ops::util::*;
use std::io::prelude::*;
use std::net::{TcpStream, UdpSocket};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use log::{info, error};
use std::env;


#[derive(Serialize, Deserialize)]
struct NetworkTaskLog {
    timestamp: String,
    user: String,
    dest_addr: String,
    dest_port: u16,
    src_addr: String,
    src_port: u16,
    bytes_sent: u64,
    protocol: String,
    process_name: String,
    process_args: String,
    process_id: u32
}

pub fn send_data(host: &String, port: u16, proto: String, data: &[u8]) -> std::io::Result<()>{
    let src_ip: String;
    let src_port: String;
    if proto == "udp" {
        let socket = UdpSocket::bind(("127.0.0.1", 12345))?;
        socket.send_to(data, (host.clone(), port))?;
        src_ip = socket.local_addr().unwrap().ip().to_string();
        src_port = socket.local_addr().unwrap().port().to_string();
    }
    else if proto == "tcp" {
        let mut stream = TcpStream::connect((host.clone(), port))?;
        stream.write(data)?;
        src_ip = stream.local_addr().unwrap().ip().to_string();
        src_port = stream.local_addr().unwrap().port().to_string();
    }
    else {
        error!("Unsupported protocol: '{}'", proto);
        return Err(std::io::Error::last_os_error());
    }
    let network_log = json!({
        "timestamp": SystemTime::now(),
        "user": get_username(),
        "dest_addr": host,
        "dest_port": port,
        "src_addr": src_ip,
        "src_port": src_port,
        "bytes_sent": data.len(),
        "protocol": proto,
        "process_name": env::args().nth(0),
        "process_args": get_commandline_args(),
        "process_id": std::process::id()});
    info!("{}", network_log);
    return Ok(());
}