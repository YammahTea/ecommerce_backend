use bcrypt::{BcryptResult};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::models::auth::{AuthConfig, Claims};
use crate::repositories::user_repo;
use crate::models::error::{AuthMiddlewareError, UserCreationError, UserLoginError};


fn hash_password(password: &str, auth_config: &AuthConfig) -> BcryptResult<String> {
    let bcrypt_cost:u32 = auth_config.bcrypt_cost;
    bcrypt::hash(&password, bcrypt_cost)
}

fn verify_password(password: &str, stored_hashed_password: &str) -> BcryptResult<bool> {
    bcrypt::verify(password, stored_hashed_password)
}

fn looks_like_email(mail: &str) -> bool {
    mail.contains("@")
}

fn create_access_token(user_id: Uuid, auth_config: &AuthConfig) -> Result<String, UserLoginError> {


    let expiration: usize = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(auth_config.access_token_expiration_time))
        .expect("Invalid timestamp.")
        .timestamp() as usize;

    let claims: Claims = Claims {
        sub: user_id.to_string(),
        exp: expiration
    };

    let secret = &auth_config.jwt_secret;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ).map_err(|error_message| {
        eprintln!("Error occurred while creating the token: {}", error_message);
        UserLoginError::TokenCreationError
    })?;

    Ok(token)
}

pub fn verify_access_token(token: &str) -> Result<Claims, AuthMiddlewareError>  {
    let secret = AuthConfig::default().jwt_secret;

    match decode(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default()) {
        Err(_) => Err(AuthMiddlewareError::InvalidToken),

        Ok(token_data) => {
            Ok(token_data.claims)
        }
    }

}


pub async fn register_user(pool: &Pool<Postgres>, auth_config: AuthConfig, user_email:String, user_password: String) -> Result<String, UserCreationError> {

    let hashed_password = {
        match hash_password(&user_password, &auth_config) {
            Ok(result) => result,
            Err(error_message) => {
                eprintln!("Error occurred while hashing the password in services/user_service: {}", error_message);
                return Err(UserCreationError::HashingError)
            }
        }
    };

    user_repo::create_user(&pool, &user_email, &hashed_password).await
}

pub async fn login_user(pool: &Pool<Postgres>, auth_config: AuthConfig, user_identifier: String, user_password: String) -> Result<String, UserLoginError> {

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
                        let _access_token = create_access_token(valid_user.id, &auth_config)?;
                        Ok(_access_token)
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