use axum::http::StatusCode;
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_access_token(email: &str) -> Result<String, StatusCode> {
    let secret: String = env::var("ACCESS_TOKEN_SECRET")
        .unwrap_or("this-is-secret".to_string())
        .to_string();
    let now = Utc::now();
    let expire: TimeDelta = Duration::minutes(15);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims {
        iat,
        exp,
        email: email.to_string(),
    };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_access_token(jwt_token: &str) -> Result<TokenData<Claims>, StatusCode> {
    let secret = env::var("ACCESS_TOKEN_SECRET")
        .unwrap_or("this-is-secret".to_string())
        .to_string();
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub fn encode_refresh_token(email: &str) -> Result<String, StatusCode> {
    let secret: String = env::var("REFRESH_TOKEN_SECRET")
        .unwrap_or("this-is-secret".to_string())
        .to_string();
    let now = Utc::now();
    let expire: TimeDelta = Duration::days(1);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims {
        iat,
        exp,
        email: email.to_string(),
    };
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_refresh_token(jwt_token: &str) -> Result<TokenData<Claims>, StatusCode> {
    let secret = env::var("REFRESH_TOKEN_SECRET")
        .unwrap_or("this-is-secret".to_string())
        .to_string();
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}
