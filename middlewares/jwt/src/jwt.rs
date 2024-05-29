use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::env;
use log::error;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    email: String,
    exp: i64,
}

impl Claims {
    fn from_user(user: &User, exp_duration: Duration) -> Self {
        Self {
            email: user.email.clone(),
            exp: (Utc::now() + exp_duration).timestamp(),
        }
    }
}

const BARTENDER_JWT_SECRET: &str = "BARTENDER_JWT_SECRET";

lazy_static! {
    static ref JWT_SECRET: String = {
        match env::var(BARTENDER_JWT_SECRET) {
            Ok(secret) => secret,
            Err(_) => {
                error!("Error: {} must be set", BARTENDER_JWT_SECRET);
                std::process::exit(1);
            }
        }
    };
}

pub fn check_jwt_secret() {
    if env::var(BARTENDER_JWT_SECRET).is_err() {
        panic!("Error: {} must be set", BARTENDER_JWT_SECRET);
    }
}

pub fn get_jwt(user: &User) -> Result<String, JwtError> {
    let claims = Claims::from_user(user, Duration::minutes(60));
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
}

pub fn decode_jwt(token: &str) -> Result<User, JwtError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    Ok(User { email: token_data.claims.email })
}
