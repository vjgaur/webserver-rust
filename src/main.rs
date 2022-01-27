use std::fs::*;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream: TcpStream = stream.unwrap();

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = std::fs::read_to_string("index.html").unwrap();
    let response: String = format!(
        "HTTP/1.1 200 OK\r\n Content-Lenght: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}
