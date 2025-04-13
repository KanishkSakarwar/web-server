use std::net::TcpListener;

mod methods;
use methods::{get_response, open_page,render_shortened_page};

mod contact;
use contact::{decode_form_body, save_to_json_file};

mod shorten;
use shorten::{generate_short_code, save_to_json,decode_url_form_body,URLmap};

mod redirection;
use redirection::{find_long_url_for_code, redirect_to};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Listening at 127.0.0.1:8080");
    for stream in listener.incoming(){
        let stream = stream.expect("could not receive stream");
        let (path,method,body) = get_response(stream.try_clone().expect("Failed to clone stream"));
        
        let long_url = find_long_url_for_code(path.as_str());
        if long_url.is_some() {
            let url = long_url.unwrap();
            println!("Redirecting to: {}", url);
            redirect_to(stream.try_clone().expect("failed to clone stream"), url.as_str());
            continue;   
        }
        if path == "/shorten" && method == "POST" {
            let url = decode_url_form_body(&body).to_string();
            let short_code = generate_short_code();
            save_to_json(
                URLmap {
                    longurl: url.clone(),
                    code: format!("/{}", short_code.clone()),
                }, "data/shortened.json");
            println!("Shortened URL: {} -> {}", url, short_code);
            render_shortened_page(short_code.as_str(),stream.try_clone().expect("Failed to clone stream"));
            continue;
        }
        if path == "/contact" && method == "POST" {
            let form_data = decode_form_body(&body);
                //println!("Parsed: {:?}", form_data);
            save_to_json_file(&form_data, "data/submissions.json");
        }
        open_page(path.as_str(), stream.try_clone().expect("Failed to clone stream"));    
    }
}
