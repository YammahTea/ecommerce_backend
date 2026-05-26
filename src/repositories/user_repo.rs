use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Transaction};
use tracing::{error, instrument};
use uuid::Uuid;
use crate::errors::user_error::{UserCreationError, UserLoginError};
use crate::models::user::{RefreshTokenInfo, User};

// NOTE: using instrument, please skip everything, fields are logged in the "services" layer ONLY

#[instrument(skip(pool, user_hashed_password, user_email))]
pub async fn create_user (pool: &Pool<Postgres>,
                          user_email: &str,
                          user_hashed_password: &str
) -> Result<String, UserCreationError> {


    let query = r#"INSERT INTO users (email, hashed_password) VALUES ($1, $2)"#;

    let result = sqlx::query(query)
        .bind(&user_email)
        .bind(&user_hashed_password)
        .fetch_optional(pool).await;

    match result {
        Ok(_) => Ok("User created successfully.".to_string()),

        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(UserCreationError::UserAlreadyExists)
        },

        Err(error_message) => {
            error!(error = ?error_message, "Error occurred while creating user");
            Err(UserCreationError::DatabaseError)
        }
    }
}

#[instrument(skip(pool, user_email))]
pub async fn get_user_by_email(pool: &Pool<Postgres>, user_email: &str) -> Result<Option<User>, UserLoginError> {
    let query = r#"SELECT * FROM users WHERE email = $1"#;

    sqlx::query_as::<_, User>(query)
        .bind(&user_email)
        .fetch_optional(pool).await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while fetching user by email");
            UserLoginError::DatabaseError
        })
}

#[instrument(skip(pool, username))]
pub async fn get_user_by_username(pool: &Pool<Postgres>, username: &str) -> Result<Option<User>, UserLoginError> {
    let query = r#"SELECT * FROM users WHERE username = $1"#;

    sqlx::query_as::<_, User>(query)
        .bind(&username)
        .fetch_optional(pool).await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while fetching user by username");
            UserLoginError::DatabaseError
        })
}

#[instrument(skip(tx, user_id, token_hash, expire_time))]
pub async fn insert_refresh_token(tx: &mut Transaction<'_, Postgres>, user_id: Uuid, token_hash: &String, expire_time: DateTime<Utc>) -> Result<(), UserLoginError> {

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user_id,
        token_hash,
        expire_time
    )
        .execute(&mut **tx).await
        .map(|_| Ok(())) // result is not needed (it will say "inserted 1 row")
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while inserting refresh token");
            UserLoginError::DatabaseError
        })?

}

#[instrument(skip(pool, token_hash))]
pub async fn find_refresh_token(pool: &Pool<Postgres>, token_hash: &String) -> Result<Option<RefreshTokenInfo>, UserLoginError> {

    let query = r#"
        SELECT r.token_id, r.user_id, u.role
        FROM refresh_tokens r

        JOIN users u
            ON u.id = r.user_id

        WHERE token_hash = $1
        AND expires_at > NOW()
    "#;

    sqlx::query_as::<_, RefreshTokenInfo>(query)
        .bind(&token_hash)
        .fetch_optional(pool).await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while fetching refresh token");
            UserLoginError::DatabaseError
        })
}

#[instrument(skip(tx, token_id))]
pub async fn delete_refresh_token(tx: &mut Transaction<'_, Postgres>, token_id: Uuid) -> Result<(), UserLoginError> {

    sqlx::query!(
        "DELETE FROM ONLY ( refresh_tokens ) WHERE token_id = $1",
        token_id
    )
        .execute(&mut **tx).await
        .map(|_| Ok(())) // result is not needed (it will say "deleted 1 row")
        .map_err(|error_message| {
            error!(error = ?error_message, token_id = ?token_id, "Error occurred while deleting refresh token");
            UserLoginError::DatabaseError
        })?

}