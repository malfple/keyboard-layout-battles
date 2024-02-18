use std::time;
use crate::error::AppError;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize};

const BCRYPT_COST: u32 = 8;
const TOKEN_ISSUER: &str = "klb-backend";
const TOKEN_EXP: u64 = time::Duration::from_secs(24 * 60 * 60).as_millis() as u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: u64,
    pub iss: String,
    pub sub: String,
    pub sub_id: u64,
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed_password = bcrypt::hash(password, BCRYPT_COST)?;
    
    Ok(hashed_password)
}

pub fn generate_access_token(user_id: u64, username: &str, token_secret: &str) -> String {
    let claims = TokenClaims{
        exp: SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_millis() as u64 + TOKEN_EXP,
        iss: String::from(TOKEN_ISSUER),
        sub: String::from(username),
        sub_id: user_id,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(token_secret.as_ref())).expect("should be no error in generating jwt");

    token
}

pub fn validate_access_token(token: &str, token_secret: &str) -> Result<TokenData<TokenClaims>, AppError> {
    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(token_secret.as_ref()),
        &Validation::new(Algorithm::HS256))?;

    // TODO: validate claims

    Ok(token_data)
}
