use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};

fn open_page(path: &str,mut stream: TcpStream){
    let kan = match path {
        "/home" => "index.txt",
        "/about" => "about.txt",
        "/contact" => "contact.txt",
        _ => "404.txt",
    };
    let body = std::fs::read_to_string(kan).unwrap_or_else(|_| "<h1>File not found</h1>".to_string());

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write_all(response.as_bytes()).expect("Failed to write to stream");
}

fn get_response(mut stream: TcpStream) -> String {
    let mut buffer = [0u8; 512];
    stream.read(&mut buffer).expect("Failed to read from stream");

    let request = String::from_utf8_lossy(&buffer);

    let request_line = request.lines().next().unwrap_or("");

    println!("Request line: {}", request_line);

    let mut parts = request_line.split_whitespace();
    
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    println!("Method: {}", method);
    println!("Path: {}", path);

    path.to_string()
}

fn _handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    match stream.read(&mut buffer) {
        Ok(n) => {
            println!("Received {} bytes:", n);
            println!("{}", String::from_utf8_lossy(&buffer[..n]));
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Listening at 127.0.0.1:8080");
    for stream in listener.incoming(){
        let stream = stream.expect("could not receive stream");
        let path = get_response(stream.try_clone().expect("Failed to clone stream"));
        open_page(path.as_str(), stream.try_clone().expect("Failed to clone stream"));
    }
}
