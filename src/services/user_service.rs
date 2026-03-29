use bcrypt::{BcryptResult};
use sqlx::{Pool, Postgres};
use crate::repositories::user_repo;
use crate::models::error::{UserCreationError, UserLoginError};

pub const BCRYPT_COST: u32 = 12;

fn hash_password(password: &str) -> BcryptResult<String> {
    bcrypt::hash(&password, BCRYPT_COST)
}

fn verify_password(password: &str, stored_hashed_password: &str) -> BcryptResult<bool> {
    bcrypt::verify(password, stored_hashed_password)
}

fn looks_like_email(mail: &str) -> bool {
    mail.contains("@")
}

pub async fn register_user(pool: Pool<Postgres>, user_email:String, user_password: String) -> Result<String, UserCreationError> {

    let hashed_password = {
        match hash_password(&user_password) {
            Ok(result) => result,
            Err(error_message) => {
                eprintln!("Error occurred while hashing the password in services/user_service: {}", error_message);
                return Err(UserCreationError::HashingError)
            }
        }
    };

    user_repo::create_user(&pool, &user_email, &hashed_password).await
}

pub async fn login_user(pool: Pool<Postgres>, user_identifier: String, user_password: String) -> Result<String, UserLoginError> {

    let user = {
        if looks_like_email(user_identifier.as_str()) {
            user_repo::get_user_by_email(&pool, user_identifier.as_str()).await?
        }
        else { user_repo::get_user_by_username(&pool, user_identifier.as_str()).await? }
    };

    match user {
        None => { Err(UserLoginError::InvalidCredentials) }

        Some(valid_user) => {
            match verify_password(user_password.as_str(), valid_user.hashed_password.as_str()) {
                Ok(result) => {
                    if result {
                        // TODO: Implement JWT token
                        Ok("Login successful".to_string())
                    }
                    else {
                        Err(UserLoginError::InvalidCredentials)
                    }
                },
                Err(e) => {
                    eprintln!("Something went wrong while verifying the password: {}", e);
                    Err(UserLoginError::InvalidCredentials)
                }
            }
        }
    }


}