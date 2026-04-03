use std::env;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub bcrypt_cost: u32,
    pub access_token_expiration_time: i64
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env"),

            bcrypt_cost: env::var("BCRYPT_COST").expect("BCRYPT_COST is not set in .env").parse().unwrap(),

            access_token_expiration_time: env::var("ACCESS_TOKEN_EXPIRE").expect("ACCESS_TOKEN_EXPIRE is not set in .env").parse().unwrap()
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize
}