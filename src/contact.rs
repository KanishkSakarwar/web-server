use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};
use url::form_urlencoded;
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    pub email: String,
    pub message: String,
    pub name: String,
}

pub fn decode_form_body(body: &str) -> Contact {
    let parsed: HashMap<String, String> = form_urlencoded::parse(body.as_bytes())
        .into_owned()
        .collect();

    Contact {
        email: parsed.get("email").cloned().unwrap_or_default(),
        message: parsed.get("message").cloned().unwrap_or_default(),
        name: parsed.get("name").cloned().unwrap_or_default(),
    }
}

pub fn save_to_json_file(contact: &Contact, file_path: &str) {
    let mut file = OpenOptions::new().create(true).append(true).open(file_path).unwrap();

    let mut content = String::new();
    let mut read_file = File::open(file_path).unwrap_or_else(|_| File::create(file_path).unwrap());
    read_file.read_to_string(&mut content).unwrap();

    let mut contacts: Vec<Contact> = if content.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    };

    contacts.push((*contact).clone());

    let updated_json = serde_json::to_string_pretty(&contacts).unwrap();

    file.set_len(0).unwrap();
    file.write_all(updated_json.as_bytes()).expect("Failed to write JSON to file");
}
