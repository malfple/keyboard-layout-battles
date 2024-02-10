use std::time;
use crate::error::AppError;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

const BCRYPT_COST: u32 = 8;
const TOKEN_ISSUER: &str = "klb-backend";
const TOKEN_EXP: u64 = time::Duration::from_secs(24 * 60 * 60).as_millis() as u64;

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    exp: u64,
    iss: String,
    sub: String,
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed_password = bcrypt::hash(password, BCRYPT_COST)?;
    
    Ok(hashed_password)
}

pub fn generate_access_token(username: &str, token_secret: &str) -> String {
    let claims = TokenClaims{
        exp: SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_millis() as u64 + TOKEN_EXP,
        iss: TOKEN_ISSUER.into(),
        sub: username.into(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(token_secret.as_ref())).expect("should be no error in generating jwt");

    token
}