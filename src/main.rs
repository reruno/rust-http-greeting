use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server running on http://0.0.0.0:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("Got new connection, handling it...");
                handle_conn(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}


fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer[..]);

    let (status_line, content) = if request_str.starts_with("GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", include_str!("index.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND","<h1>404 - Not Found</h1>")
    };

    let length = content.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
    );

    stream.write_all(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}