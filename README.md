# Rust URL Shortener Web Server

This is a simple **URL shortener** web server built with **Rust**. The server allows users to shorten long URLs into compact, easy-to-share links and redirects users to the original URLs when accessed. It uses a JSON file for storing the URL mappings persistently.

## Features:
- **URL Shortening**: Converts long URLs into unique short codes.
- **Redirection**: Shortened URLs redirect users to the original long URL.
- **Persistent Storage**: URL mappings are stored in a JSON file for persistence.
- **Simple Frontend**: A lightweight HTML interface that allows users to input a long URL, generate a short URL, and copy it to the clipboard.
- **Efficient Handling**: Built with Rust's standard library and handling HTTP requests directly via `TcpListener` and `TcpStream`.

## Technologies Used:
- **Rust**: The systems programming language known for its speed and memory safety.
- **Serde**: For serializing and deserializing data to and from JSON format.
- **rand**: Used to generate unique short codes for URLs.
- **Standard Library**: Uses Rust's standard `TcpListener` and `TcpStream` to handle HTTP requests without external web frameworks.

## Setup:
1. Clone the repository:  
   `git clone https://github.com/your-username/your-repo-name.git`
2. Install Rust and dependencies (if not already installed).
3. Run the server:  
   `cargo run`
4. Visit the server in your browser:  
   `http://localhost:8080`
