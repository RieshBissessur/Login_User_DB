mod utils;
mod models;

use chrono::{Local, DateTime, Duration};
use crypto_hash::{hex_digest, Algorithm};
use rand::Rng;
use uuid::Uuid;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use tokio;
use warp::{reject, Filter, Rejection, Reply};
use utils::*;
use models::*;

const APP_VERSION: &'static f32 = &0.1;

#[tokio::main]
async fn main() {
    if !fs::metadata("./Json").is_ok() {
        fs::create_dir("./Json").expect("Failed to create Json directory");
    }
    if !fs::metadata("./Json/Users").is_ok() {
        fs::create_dir("./Json/Users").expect("Failed to create User directory");
    }

    add_routes().await;
}

async fn handle_get_health() -> Result<impl Reply, Rejection> {
    return Ok(warp::reply::with_status(warp::reply(), warp::http::StatusCode::OK));
}

async fn handle_custom_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    if let Some(custom_error) = err.find::<CustomRejection>() {
        // Handle the custom rejection and return a 400 Bad Request response
        let response = warp::reply::with_status(
            warp::reply::html(format!("Bad Request: {:?}", custom_error)),
            warp::http::StatusCode::BAD_REQUEST,
        );
        Ok(response)
    } else {
        // For other rejections, return a generic 500 Internal Server Error response
        Ok(warp::reply::with_status(
            warp::reply::html("Internal Server Error".to_string()),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

async fn handle_register(user_data: RegisterUser) -> Result<impl Reply, Rejection> {
    match read_user_data(&user_data.username.to_lowercase()){
        Ok(_) => return Err(warp::reject::custom(CustomRejection("Username exists".to_string()))),
        Err(_) => {},
    };

    // Read the existing user map
    let mut user_map: HashMap<String, String> = match read_usermap(){
        Ok(user_map) => user_map,
        Err(err) => return Err(warp::reject::custom(CustomRejection(err))),
    };

    // Check if the username or email exists in the user map
    if user_map.contains_key(&user_data.username.to_lowercase()) || user_map.values().any(|v| v.to_lowercase() == user_data.email.to_lowercase()){
        return Err(reject::custom(CustomRejection("Username or email already associated with an account".to_string())));
    } else {
        user_map.insert(user_data.email.to_lowercase().to_string(), user_data.username.to_lowercase().to_string());
         match write_usermap(&user_map){
            Ok(_) => {},
            Err(err) => return Err(warp::reject::custom(CustomRejection(err))),
        };
    }

    let mut rng = rand::thread_rng();
    let guid = Uuid::from_u128(rng.gen()).as_u128();
    let password_hash = hex_digest(Algorithm::SHA256, user_data.password.as_bytes());
    let full_user_data = FullUserData {
        username: user_data.username.clone(),
        password: password_hash,
        email: Some(user_data.email),
        guid,
        avatar: None,
    };

    match write_user_data(full_user_data){
        Ok(_) => {
            let response = LoginResponse { session_key: String::new(), username: user_data.username};
            return Ok(warp::reply::json(&response))
        }
        Err(_) => return Err(warp::reject::custom(CustomRejection("Internal Error01".to_string()))),
    };
}

async fn handle_login(login: LoginRequest) -> Result<impl Reply, Rejection> {
    let password_hash = hex_digest(Algorithm::SHA256, &login.password.as_bytes());
    let username = match email_lookup(&login.username){
        Ok(username) => username,
        Err(err) => return Err(warp::reject::custom(CustomRejection(format!("{:?}", err)))),
    };
    
    let user_data: FullUserData = match read_user_data(&username) {
        Ok(user_data) => user_data,
        Err(_) => return Err(warp::reject::custom(CustomRejection(format!("Incorrect username or password for this account.")))),
    };

    if login.version < APP_VERSION.to_owned() {
        return Err(reject::custom(CustomRejection("Please update application version".to_string())));
    }

    if &user_data.password == &password_hash {
        let mut rng = rand::thread_rng();
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let session_key: String = (0..32).map(|_| {
                let index = rng.gen_range(0..characters.len());
                characters.chars().nth(index).unwrap()
            })
            .collect();

        let session_data = SessionData {
            session_key: session_key.clone(),
        };

        match write_sesion_data(session_data, &username){
            Ok(_) => {},
            Err(_) => return Err(reject::custom(CustomRejection("Unable to save session data".to_string()))),
        };

        return Ok(warp::reply::json(&LoginResponse {session_key, username}));
    } else {
        return Err(reject::custom(CustomRejection("Incorrect username or password for this account.".to_string())));
    }
}

async fn handle_user_data_retrieval(requset_data: UserDataRequest) -> Result<impl Reply, Rejection> {
    let session_data = match read_session_data(&requset_data.username){
        Ok(session_data) => session_data,
        Err(_) => return Err(reject::custom(CustomRejection("Can not read authentication key".to_string()))),
    };

    if session_data.session_key != requset_data.session_key {
        return Err(reject::custom(CustomRejection("Incorrect authentication key".to_string())))
    } else {
        let user_data: FullUserData = match read_user_data(&requset_data.username){
            Ok(user_data) => user_data,
            Err(_) => return Err(reject::custom(CustomRejection("Unable to read to user data".to_string()))),
        };

        let user = UserData {
            username: user_data.username,
            email: user_data.email,
            avatar: user_data.avatar,
        };

        return Ok(warp::reply::json(&user))
    }
}

async fn handle_user_data_update(requset_data: UserDataUpdate) -> Result<impl Reply, Rejection> {
    let session_data = match read_session_data(&requset_data.username){
        Ok(session_data) => session_data,
        Err(_) => return Err(reject::custom(CustomRejection("Can not read authentication key".to_string()))),
    };

    if session_data.session_key != requset_data.session_key {
        return Err(reject::custom(CustomRejection("Incorrect authentication key".to_string())))
    } else {
        let user_data: FullUserData = match read_user_data(&requset_data.username){
            Ok(user_data) => user_data,
            Err(_) => return Err(reject::custom(CustomRejection("Unable to read to user data".to_string()))),
        };

        let new_user = match requset_data.new_username{
            Some(new_user) => new_user,
            None => user_data.username,
        };

        let email = match requset_data.email{
            Some(email) => Some(email),
            None => user_data.email,
        };

        let avatar = match requset_data.avatar{
            Some(avatar) => Some(avatar),
            None => user_data.avatar,
        };

        let user = UserData {
            username: new_user,
            email: email,
            avatar: avatar,
        };

        return Ok(warp::reply::json(&user))
    }
}

async fn request_password_reset(req: RequestPassword) -> Result<impl Reply, Rejection> {
    let username = match email_lookup(&req.email){
        Ok(username) => username,
        Err(_) => return Err(warp::reject::custom(CustomRejection("Failed to find email".to_string()))),
    };

    let otp_string: String = (0..4)
        .map(|_| rand::thread_rng().gen_range(0..=9).to_string())
        .collect();

    let otp_data = OTPData {
        otp: otp_string.clone(),
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    match write_otp_data(otp_data, &username){
        Ok(_) => {},
        Err(_) => return Err(warp::reject::custom(CustomRejection("Failed to write otp data".to_string()))),
    };

    match send_otp(&otp_string, &username, &req.email).await{
        Ok(_) => {},
        Err(_) => return Err(warp::reject::custom(CustomRejection("Failed to send otp data".to_string()))),
    };

    return Ok(warp::reply::json(&format!("OTP Sent to email address")));
}

async fn check_otp(req: OTPSubmit) -> Result<impl Reply, Rejection> {
    let username = match email_lookup(&req.email){
        Ok(username) => username,
        Err(_) => return Err(warp::reject::custom(CustomRejection("Failed to find email".to_string()))),
    };

    let otp_data: OTPData = match read_otp_data(&username){
        Ok(otp_data) => otp_data,
        Err(_) => return Err(warp::reject::custom(CustomRejection("Failed to read otp data".to_string()))),
    };

    let input_datetime = match DateTime::parse_from_str(&otp_data.date, "%Y-%m-%d %H:%M:%S"){
        Ok(input_datetime) => input_datetime,
        Err(_) => return Err(warp::reject::custom(CustomRejection("Failed to read otp date".to_string()))),
    };

    let current_datetime = Local::now();
    let duration = match Duration::try_hours(2) {
        Some(duration) => duration,
        None => return Err(warp::reject::custom(CustomRejection("Failed to read otp date".to_string()))),
    };

    if current_datetime.signed_duration_since(input_datetime) < duration && &otp_data.otp == &req.otp {
        let mut user_data: FullUserData = match read_user_data(&username){
            Ok(user_data) => user_data,
            Err(_) => return Err(reject::custom(CustomRejection("Unable to read to user data".to_string()))),
        };

        let password_hash = hex_digest(Algorithm::SHA256, req.password.as_bytes());
        user_data.password = password_hash;

        match write_user_data(user_data){
            Ok(_) => return Ok(warp::reply::json(&format!("OTP match and valid"))),
            Err(_) => return Err(warp::reject::custom(CustomRejection("Internal Error01".to_string()))),
        };        
    } else {
        return Ok(warp::reply::json(&format!("OTP invalid or expired")));
    }
}

async fn add_routes(){
    let get_health = warp::get()
    .and(warp::path("health"))
    .and_then(handle_get_health);

    let register_user = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and_then(handle_register);
    
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(handle_login);

    let retrieve_user_data = warp::post()
        .and(warp::path("user_data"))
        .and(warp::body::json())
        .and_then(handle_user_data_retrieval);

    let reset_request = warp::post()
        .and(warp::path("reset_request"))
        .and(warp::body::json())
        .and_then(request_password_reset);

    let otp_check = warp::post()
        .and(warp::path("check_otp"))
        .and(warp::body::json())
        .and_then(check_otp);

    let update_user_data = warp::post()
        .and(warp::path("update_user_data"))
        .and(warp::body::json())
        .and_then(handle_user_data_update);

    // Combine filters and run the server
    let routes = register_user
        .or(login)
        .or(retrieve_user_data)
        .or(update_user_data)
        .or(reset_request)
        .or(otp_check)
        .or(get_health)
        .recover(handle_custom_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}