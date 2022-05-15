use fs::read_to_string;
use std::{fs, thread};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    println!("response {:?}", (status, filename));

    let contents = read_to_string(filename).unwrap();
    let response = format!("{}Content-Length: {}\r\n\r\n{}", status, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


