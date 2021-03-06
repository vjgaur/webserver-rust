use std::fs::*;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream: TcpStream = stream.unwrap();
        pool.execute(|| {
            handle_connection(_stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let sleep: &[u8; 21] = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK,", "index.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK,", "index.html")
    } else {
        ("HTTP/1.1 404 Not Found,", "404.html")
    };

    let contents: String = std::fs::read_to_string(filename).unwrap();
    let response: String = format!(
        "{}\r\n Content-Lenght: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    //    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}
