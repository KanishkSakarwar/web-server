use std::net::{TcpListener,TcpStream};
use std::io::Read;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, 
            Ok(n) => {
                println!("Received {} bytes:", n);
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Listening at 127.0.0.1:8080");
    for stream in listener.incoming(){
        handle_client(stream.expect("could not receive stream"));
    }
}
