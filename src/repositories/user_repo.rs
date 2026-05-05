use sqlx::{Pool, Postgres};
use tracing::{error, instrument};
use crate::errors::user_error::{UserCreationError, UserLoginError};
use crate::models::user::User;

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
        .fetch_optional(pool)
        .await;

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
        .fetch_optional(pool)
        .await
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
        .fetch_optional(pool)
        .await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while fetching user by username");
            UserLoginError::DatabaseError
        })
}