use std::fs;
use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct URLmap {
    pub longurl: String,
    pub code: String,
}

pub fn find_long_url_for_code(code: &str) -> Option<String> {
    let data = fs::read_to_string("data/shortened.json").ok()?;
    let entries: Vec<URLmap> = serde_json::from_str(&data).ok()?;
    
    for entry in entries {
        if entry.code == code {
            return Some(entry.longurl);
        }
    }
    None
}

pub fn redirect_to(mut stream: TcpStream, location: &str) {
    let response = format!(
        "HTTP/1.1 302 Found\r\nLocation: {}\r\nContent-Length: 0\r\n\r\n",
        location
    );
    stream.write_all(response.as_bytes()).unwrap();
}