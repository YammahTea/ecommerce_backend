use std::env;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub refresh_secret: String,
    pub bcrypt_cost: u32,
    pub access_token_expiration_time: i64,
    pub refresh_token_expiration_time: i64
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env"),

            refresh_secret: env::var("REFRESH_SECRET").expect("REFRESH_SECRET is not set in .env"),

            bcrypt_cost: env::var("BCRYPT_COST").expect("BCRYPT_COST is not set in .env").parse().unwrap(),

            access_token_expiration_time: env::var("ACCESS_TOKEN_EXPIRE").expect("ACCESS_TOKEN_EXPIRE is not set in .env").parse().unwrap(),

            refresh_token_expiration_time: env::var("REFRESH_TOKEN_EXPIRE").expect("REFRESH_TOKEN_EXPIRE is not set in .env").parse().unwrap()

        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Claims {
    pub(crate) sub: Uuid,
    pub(crate) role: String,
    pub(crate) exp: usize
}