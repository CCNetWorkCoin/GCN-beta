
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Received from client: {}", String::from_utf8_lossy(&buffer));
    stream.write_all(b"Welcome to GreenCompute, star!").unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("GreenCompute Server running on 144.217.82.140:8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New star connected: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}
