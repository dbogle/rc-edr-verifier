use std::io::prelude::*;
use std::net::{TcpStream, UdpSocket};

pub fn send_data(host: &str, port: u16, proto: &str, data: &[u8]) {
    if proto == "udp" {
        let socket = UdpSocket::bind(("127.0.0.1", 12345)).expect("couldn't bind to address");
        socket.send_to(data, (host, port)).expect("Failed to send data to host");
    }
    else if proto == "tcp" {
        let mut stream = TcpStream::connect((host, port)).expect("Failed to connect to host");
        stream.write(data).expect("Failed to send data to host");
    }
    else {
        println!("Unsupported protocol: '{}'", proto);
    }
    
}