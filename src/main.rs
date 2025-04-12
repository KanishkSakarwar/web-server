use std::net::TcpListener;

mod methods;
use methods::{get_response, open_page, decode_form_body, save_to_json_file};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Listening at 127.0.0.1:8080");
    for stream in listener.incoming(){
        let stream = stream.expect("could not receive stream");
        let (path,method,body) = get_response(stream.try_clone().expect("Failed to clone stream"));
        println!("{} - {}",path,method);
        if path == "/contact" && method == "POST" {
            let form_data = decode_form_body(&body);
            println!("Parsed: {:?}", form_data);
            save_to_json_file(&form_data, "submissions.json");
        }
        open_page(path.as_str(), stream.try_clone().expect("Failed to clone stream"));
    }
}
