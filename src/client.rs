use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("144.217.82.140:8080").unwrap();
    println!("Connected to GreenCompute Server!");

    stream.write_all(b"Hello from WSL star!").unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Server says: {}", String::from_utf8_lossy(&buffer));
}
