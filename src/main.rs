use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Initialize the server.
    let address = "localhost:8000";
    let listener = TcpListener::bind(address).unwrap();

    println!("Server initilized on {}.", address);

    // Listen for incoming connections.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

// Handle the incoming connection.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Stream received.");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET /home HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        send_home(stream);
    } else {
        send_404(stream);
    }
}

fn build_response(content: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}

fn send_home(mut stream: TcpStream) {
    let content = fs::read_to_string("home.html").unwrap();

    stream.write(build_response(content).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn send_404(mut stream: TcpStream) {
    let content = fs::read_to_string("404.html").unwrap();

    stream.write(build_response(content).as_bytes()).unwrap();
    stream.flush().unwrap();
}
