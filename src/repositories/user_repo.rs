use sqlx::{Pool, Postgres};
use crate::models::error::UserCreationError;

pub async fn create_user (pool: Pool<Postgres>,
                          user_email: &str,
                          user_hashed_password: &str
) -> Result<String, UserCreationError> {


    let query = r"INSERT INTO users (email, hashed_password) VALUES ($1, $2)";

    let result = sqlx::query(query)
        // note: user ID is already generated in users table (check migration create_users_table.sql file)
        .bind(&user_email)
        .bind(&user_hashed_password)
        .fetch_optional(&pool)
        .await;

    match result {
        Ok(_) => Ok("User created successfully.".to_string()),

        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(UserCreationError::UserAlreadyExists)
        },

        Err(e) =>{
            eprintln!("Error occurred while creating user in repositories/user_repo.rs: {}", e);
            Err(UserCreationError::DatabaseError)
        }
    }
}