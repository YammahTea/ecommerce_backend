use bcrypt::{BcryptResult};
use sqlx::{Pool, Postgres};
use crate::repositories::user_repo;
pub const BCRYPT_COST: u32 = 12;

fn hash_password(password: &str) -> BcryptResult<String> {
    bcrypt::hash(&password, BCRYPT_COST)
}

pub async fn register_user(pool: Pool<Postgres>, user_email:String, user_password: String) -> Result<String, String> {

    let hashed_password = {
        match hash_password(&user_password) {
            Ok(result) => result,
            Err(error_message) => {
                eprintln!("Error occurred while hashing the password in services/user_service: {}", error_message);
                return Err("Something went wrong while hashing the password".to_string())
            }
        }
    };

    user_repo::create_user(pool, &user_email, &hashed_password).await
}