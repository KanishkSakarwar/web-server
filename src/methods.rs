use std::net::TcpStream;
use std::io::{Read,Write};

pub fn render_shortened_page(code: &str,mut stream: TcpStream){
    let html = std::fs::read_to_string("public/shorten.txt").unwrap_or_else(|_| "<h1>File not found</h1>".to_string());
    let short_url = format!("http://localhost:8080/{}", code);
    let html = html.replace("{short_url}", &short_url);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        html.len(),
        html
    );
    stream.write_all(response.as_bytes()).expect("Failed to write to stream");
}

pub fn open_page(path: &str,mut stream: TcpStream){
    let kan = match path {
        "/about" => "public/about.txt",
        "/contact" => "public/contact.txt",
        "/index" | "/" | "/home" => "public/index.txt",
        "/shorten"  => "public/shorten.txt",
        _ => "404.txt",
    };
    if path == "/shorten" {

    }
    else{
        let body = std::fs::read_to_string(kan).unwrap_or_else(|_| "<h1>File not found</h1>".to_string());

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).expect("Failed to write to stream");
    }
}

pub fn get_response(mut stream: TcpStream) -> (String, String, String) {
    let mut buffer = Vec::new();
    let mut temp_buffer = [0u8; 512];

    loop {
        let bytes_read = stream.read(&mut temp_buffer).expect("Failed to read from stream");
        if bytes_read == 0 {
            break;
        }
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);
        if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    let request = String::from_utf8_lossy(&buffer);

    let request_line = request.lines().next().unwrap_or("");
    println!("Request line: {}", request_line);

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    println!("Method: {}", method);
    println!("Path: {}", path);

    let body = if let Some(pos) = request.find("\r\n\r\n") {
        let body_start = pos + 4;
        request[body_start..].to_string()
    } else {
        "".to_string()
    };

    println!("Body: {}", body);

    (path.to_string(), method.to_string(), body)
}



pub fn _handle_client(mut stream: TcpStream) {
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