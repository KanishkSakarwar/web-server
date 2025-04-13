use std::fs::{OpenOptions,File};
use std::io::{Read,Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct URLmap {
    pub longurl: String,
    pub code: String,
}

use rand::distr::{Alphanumeric,SampleString};
use std::collections::HashMap;
use url::form_urlencoded;

pub fn generate_short_code() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 6)
}

pub fn save_to_json(contact: URLmap, file_path: &str) {
    let mut file = OpenOptions::new().create(true).append(true).open(file_path).unwrap();

    let mut content = String::new();
    let mut read_file = File::open(file_path).unwrap_or_else(|_| File::create(file_path).unwrap());
    read_file.read_to_string(&mut content).unwrap();

    let mut contacts: Vec<URLmap> = if content.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    };

    contacts.push((contact).clone());

    let updated_json = serde_json::to_string_pretty(&contacts).unwrap();

    file.set_len(0).unwrap();
    file.write_all(updated_json.as_bytes()).expect("Failed to write JSON to file");
}

pub fn decode_url_form_body(body: &str) -> String {
    let parsed: HashMap<String, String> = form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect();

    match parsed.get("url") {
        Some(url) if !url.is_empty() => url.clone(),
        _ => {
            eprintln!("Error: 'longurl' key not found or empty in the body: {}", body);
            String::new()
        }
    }
}
