use sqlx::{Pool, Postgres};
use crate::repositories::user_repo;
pub const BCRYPT_COST: u32 = 12;


fn hash_password(password: &String) -> String {
    // TODO: Handle bcrypt result without unwrap
    bcrypt::hash(&password, BCRYPT_COST).unwrap()
}

pub async fn register_user(pool: Pool<Postgres>, user_email:String, user_password: String) -> Result<String, String> {
    let hashed_password = hash_password(&user_password);

    user_repo::create_user(pool, &user_email, &hashed_password).await
}