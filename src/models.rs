use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductRequest {
    pub username: String,
    pub shared_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestPassword {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OTPSubmit {
    pub otp: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OTPData {
    pub otp: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub version: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub session_key: String,
    pub username: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDataRequest {
    pub session_key: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionData {
    pub session_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub username: String,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullUserData {
    pub username: String,
    pub guid: u128,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDataUpdate {
    pub username: String,
    pub new_username: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub session_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendGridEmail {
    pub personalizations: Vec<Personalization>,
    pub from: EmailAddress,
    pub template_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Personalization {
    pub to: Vec<EmailAddress>,
    pub dynamic_template_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress {
    pub email: String,
}

#[derive(Debug)]
pub struct CustomIoError(pub std::io::Error);

impl warp::reject::Reject for CustomIoError {}

#[derive(Debug)]
pub struct CustomRejection(pub String);

impl warp::reject::Reject for CustomRejection {}