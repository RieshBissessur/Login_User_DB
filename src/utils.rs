use reqwest::Client;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::{EmailAddress, FullUserData, OTPData, Personalization, SendGridEmail, SessionData};

pub fn write_user_data(user_data: FullUserData) -> Result<(),()> {
    let file_path = format!("./Json/Users/{}/user_data.txt", user_data.username.to_lowercase());
    if !fs::metadata(format!("./Json/Users/{}/", & user_data.username.to_lowercase())).is_ok() {
        let _  = fs::create_dir(format!("./Json/Users/{}", & user_data.username.to_lowercase()));
    }

    let serialized_user_data = match serde_json::to_string(&user_data){
        Ok(user_data) => user_data,
        Err(_) => return Err(()),
    };

    match fs::write(&file_path, serialized_user_data) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(()),
    }
}

pub fn read_user_data(username: &str) -> Result<FullUserData,()> {
    let file_path = format!("./Json/Users/{}/user_data.txt", username.to_lowercase());
    if !fs::metadata(&file_path).is_ok() {
        return Err(());
    }

    let user_str = match read_from_file(&file_path) {
        Some(data) => data,
        None => return Err(()),
    };

    match serde_json::from_str(&user_str){
        Ok(user_data) => return Ok(user_data),
        Err(_) => return Err(()),
    };
}

pub fn write_sesion_data(session_data: SessionData, username: &String) -> Result<(),()> {
    let file_path = format!("./Json/Users/{}/session_data.txt", username.to_lowercase());
    if !fs::metadata(format!("./Json/Users/{}/", & username.to_lowercase())).is_ok() {
        return Err(());
    }

    let serialized_session_data = match serde_json::to_string(&session_data){
        Ok(session_data) => session_data,
        Err(_) => return Err(()),
    };

    match fs::write(&file_path, serialized_session_data) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(()),
    }
}

pub fn read_session_data(username: &str) -> Result<SessionData,()> {
    let file_path = format!("./Json/Users/{}/session_data.txt", username.to_lowercase());
    if !fs::metadata(&file_path).is_ok() {
        return Err(());
    }

    let session_str = match read_from_file(&file_path) {
        Some(data) => data,
        None => return Err(()),
    };

    match serde_json::from_str(&session_str){
        Ok(session_data) => return Ok(session_data),
        Err(_) => return Err(()),
    };
}

pub fn write_otp_data(otp_data: OTPData, username: &String) -> Result<(),()> {
    let file_path = format!("./Json/Users/{}/otp_data.txt", username.to_lowercase());
    if !fs::metadata(format!("./Json/Users/{}/", & username.to_lowercase())).is_ok() {
        return Err(());
    }

    let serialized_otp_data = match serde_json::to_string(&otp_data){
        Ok(otp_data) => otp_data,
        Err(_) => return Err(()),
    };

    match fs::write(&file_path, serialized_otp_data) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(()),
    }
}

pub fn read_otp_data(username: &str) -> Result<OTPData,()> {
    let file_path = format!("./Json/Users/{}/otp_data.txt", username.to_lowercase());
    if !fs::metadata(&file_path).is_ok() {
        return Err(());
    }

    let otp_str = match read_from_file(&file_path) {
        Some(data) => data,
        None => return Err(()),
    };

    match serde_json::from_str(&otp_str){
        Ok(otp_data) => return Ok(otp_data),
        Err(_) => return Err(()),
    };
}

pub async fn send_otp(otp: &str, username: &str, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "API_KEY";
    let sender_email = "no-reply@gmail.com";
    let recipient_email = &email.to_string();
    let template_id = "d-36dab063ce184e4180e716439b12ac9a";

    let url = "https://api.sendgrid.com/v3/mail/send";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    let mut dynamic_template_data = HashMap::new();
    dynamic_template_data.insert("username".to_string(), username.to_string());
    dynamic_template_data.insert("otp".to_string(), otp.to_string());
    dynamic_template_data.insert("email".to_string(), email.to_string());

    let email = SendGridEmail {
        personalizations: vec![Personalization {
            to: vec![EmailAddress {
                email: recipient_email.to_string(),
            }],
            dynamic_template_data,
        }],
        from: EmailAddress {
            email: sender_email.to_string(),
        },
        template_id: template_id.to_string(),
    };

    let client = Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .body(serde_json::to_string(&email)?)
        .send()
        .await;

    match response {
        Ok(response) => {
            println!("Response: {:?}", response);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error sending email: {}", err);
            Err(Box::new(err))
        }
    }
}

pub fn email_lookup(login: &str) -> Result<String, Box<dyn std::error::Error>> {
    if login.contains("@"){
        // Read the user map file and create a HashMap of username-email mappings
        let user_map: HashMap<String, String> = match read_usermap(){
            Ok(user_map) => user_map,
            Err(_) =>return Err("Unable to read user map.".into()),
        };

        // Retrieve the username based on an email
        if let Some(username) = user_map.get(&login.to_lowercase()) {
            Ok(username.clone())
        } else {
            Err("Email not found in the user map.".into())
        }
    } else {
        Ok(login.to_string())
    }
}

pub fn read_usermap() -> Result<HashMap<String, String>, String> {
    let target_directory = Path::new("./Json/");
    let user_map_file_path = target_directory.join("user_map.txt");
    match read_from_file(&user_map_file_path.to_string_lossy().to_string()) {
        Some(hash_map_str) => {
            let parsed_data: Result<HashMap<String, String>, serde_json::Error> =
                serde_json::from_str(&hash_map_str);
            match parsed_data {
                Ok(data) => {
                    println!("Deserialized usermap");
                    return Ok(data);
                }
                Err(_) => {
                    return Err("Failed to deserialize usermap".to_string());
                }
            }
        }
        None => return Err("Could not find usermap".to_string()),
    }
}

pub fn write_usermap(usermap: &HashMap<String, String>) -> Result<String, String> {
    let target_directory = Path::new("./Json/");
    let user_map_file_path = target_directory.join("user_map.txt");

    match serde_json::to_string(&usermap) {
        Ok(user_map_string) => write_to_file(&user_map_file_path.to_string_lossy().to_string(), &user_map_string),
        Err(_) => return Err("Failed to save updated usermap".to_string()),
    };

    Ok("Saved updated user map".to_string())
}

fn read_from_file(relative_path: &String) -> Option<String> {
    if !fs::metadata(relative_path).is_ok() {
        return None;
    }

    let _data = match fs::read_to_string(relative_path) {
        Ok(data) => return Some(data),
        Err(_) => return None,
    };

}

fn write_to_file(relative_path: &String, data: &String) -> bool {
    if !fs::metadata(&relative_path).is_ok() {
        return false;
    }

    match fs::write(&relative_path, data) {
        Ok(_) => return true,
        Err(err) => {
            println!("Failed to writing data to file: {:?}", err);
            return false;
        }
    }
}
