use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation , TokenData , errors::Result};

use serde::{Serialize, Deserialize};

const SECRET: &[u8] = b"feijeo";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(sub: String, exp: usize) -> Result<String> {
    let claims = Claims { sub, exp};
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default())
}