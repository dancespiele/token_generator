use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub email: String,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let expire_token: DateTime<Utc> = Utc::now() + Duration::days(1);

    let secret = args.get(1).unwrap();
    let email: &String = args.get(2).unwrap();
    let claims = Claims {
        sub: String::from("SYSTEM"),
        iss: String::from("system"),
        email: email.to_string(),
        role: String::from("SYSTEM"),
        iat: Utc::now().timestamp(),
        exp: expire_token.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    println!("token: {}", token);
}
