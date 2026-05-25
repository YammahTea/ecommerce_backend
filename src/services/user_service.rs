use bcrypt::{BcryptResult};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{Pool, Postgres};
use tracing::{error, info, instrument};
use uuid::Uuid;
use sha2::{Digest, Sha256};
use crate::models::auth::{AuthConfig, Claims};
use crate::repositories::user_repo;
use crate::errors::user_error::{UserCreationError, UserLoginError};
use crate::errors::middleware_error::AuthenticationMiddlewareError;
use crate::models::user::UserLoginResponse;
use crate::repositories::user_repo::{delete_refresh_token, find_refresh_token, insert_refresh_token};

// NOTE: use instrument on "async" functions ONLY
// NOTE: using instrument, fields are logged in the "services" layer ONLY

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

fn create_access_token(user_id: &Uuid, user_role: String, auth_config: &AuthConfig) -> Result<String, UserLoginError> {


    let expiration: usize = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(auth_config.access_token_expiration_time))
        .ok_or(UserLoginError::TokenCreationError)?
        .timestamp() as usize;

    let claims: Claims = Claims {
        sub: *user_id,
        role: user_role,
        exp: expiration
    };

    let secret = &auth_config.jwt_secret;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ).map_err(|error_message| {
        error!(error = ?error_message, "Error occurred while creating the token");
        UserLoginError::TokenCreationError
    })?;

    Ok(token)
}

fn create_refresh_token(auth_config: &AuthConfig) -> (String, String) {
    let raw_token = Uuid::new_v4().to_string();

    let hashed_token = hash_raw_token(&raw_token, &auth_config.refresh_secret);

    (raw_token, hashed_token)
}

fn hash_raw_token(raw_token: &String, refresh_secret: &String) -> String {
    let token_hash = Sha256::new()
        .chain_update(&raw_token)
        .chain_update(&refresh_secret)
        .finalize();

    hex::encode(token_hash)
}

// Used in the auth middleware
pub(crate) fn verify_access_token(token: &str, auth_config: &AuthConfig) -> Result<Claims, AuthenticationMiddlewareError>  {

    match decode(&token, &DecodingKey::from_secret(&auth_config.jwt_secret.as_bytes()), &Validation::default()) {
        Err(_) => Err(AuthenticationMiddlewareError::InvalidToken),

        Ok(token_data) => {
            Ok(token_data.claims)
        }
    }

}


#[instrument(skip(pool, auth_config, user_password), fields(user_email = %user_email))]
pub async fn register_user(pool: &Pool<Postgres>, auth_config: AuthConfig, user_email:String, user_password: String) -> Result<String, UserCreationError> {

    let hashed_password = {
        match hash_password(&user_password, &auth_config) {
            Ok(result) => result,
            Err(error_message) => {
                error!(err = ?error_message, "Error occurred while hashing the password");
                return Err(UserCreationError::HashingError)
            }
        }
    };

    user_repo::create_user(&pool, &user_email, &hashed_password).await
}


#[instrument(skip(pool, auth_config, user_password), fields(user_identifier = tracing::field::Empty))]
pub async fn login_user(pool: &Pool<Postgres>, auth_config: &AuthConfig, user_identifier: String, user_password: String) -> Result<UserLoginResponse, UserLoginError> {

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
                        tracing::Span::current().record("user_identifier", &valid_user.id.to_string());

                        let _access_token = create_access_token(&valid_user.id, valid_user.role, &auth_config)?;
                        let (_raw_token, hashed_token) = create_refresh_token(&auth_config);
                        
                        let refresh_token_expiration_date = chrono::Utc::now()
                            .checked_add_signed(chrono::Duration::days(auth_config.refresh_token_expiration_time))
                            .ok_or(UserLoginError::TokenCreationError)?;


                        insert_refresh_token(&pool, valid_user.id, &hashed_token, refresh_token_expiration_date).await?;
                        
                        Ok(UserLoginResponse {
                            access_token: _access_token,
                            raw_token: _raw_token
                        })
                    }

                    else { Err(UserLoginError::InvalidCredentials) }
                },
                Err(error_message) => {
                    error!(error = ?error_message, "Something went wrong while verifying the password");
                    Err(UserLoginError::InvalidCredentials)
                }
            }
        }
    }

}

pub async fn token_rotation(pool: &Pool<Postgres>, raw_token: &String, auth_config: &AuthConfig) -> Result<UserLoginResponse, UserLoginError> {

    // Logic:
    //     Hashes the raw token
    //     Searches the refresh_tokens table for that hash
    //     If it exists AND expires_at is in the future:
    //          Generate a new 15-minute Access Token.
    //          Generate a new 7-day Refresh Token.
    //          Delete the old Refresh Token from the DB.
    //          Save the new Refresh Token to the DB.
    //     Return both new tokens to the user.

    // return 401 if anything fails

    let hashed_token = hash_raw_token(&raw_token, &auth_config.refresh_secret);

    let refresh_token = find_refresh_token(&pool, &hashed_token).await?;

    match refresh_token {
        None => { Err(UserLoginError::InvalidToken) }

        Some(token_info) => {

            let _access_token = create_access_token(&token_info.user_id, token_info.role, &auth_config)?;

            let (_raw_token, hashed_token) = create_refresh_token(&auth_config);
            let refresh_token_expiration_date = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(auth_config.refresh_token_expiration_time))
                .ok_or(UserLoginError::TokenCreationError)?;

            // old refresh token
            delete_refresh_token(&pool, token_info.token_id).await?;

            insert_refresh_token(&pool, token_info.user_id, &hashed_token, refresh_token_expiration_date).await?;

            Ok(UserLoginResponse {
                access_token: _access_token,
                raw_token: _raw_token
            })

        }
    }


}