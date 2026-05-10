use std::collections::HashMap;
use uuid::Uuid;
use tracing::{error, instrument};
use redis::{AsyncCommands};
use crate::errors::cart_error::CartError;

#[instrument(skip(redis_connection, user_id))]
pub async fn get_cart(redis_connection: &mut deadpool_redis::Connection, user_id: &Uuid) -> Result<HashMap<String, i32>, CartError> {
    let cart_key: String = format!("cart:{}", user_id);

    let cart: HashMap<String, i32> =  redis_connection
        .hgetall(&cart_key)
        .await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while fetching cart by user_id");
            CartError::RedisKeyError
        })?;

    Ok(cart)
}

#[instrument(skip(redis_connection, user_id))]
pub async fn clear_cart(redis_connection: &mut deadpool_redis::Connection, user_id: Uuid) -> Result<(), CartError> {
    let cart_key: String = format!("cart:{}", user_id);

    redis_connection.del::<_, ()>(&cart_key)
        .await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while clearing cart");
            CartError::RedisKeyError
        })

    // NOTE: return type is the number of keys deleted, keep it empty
}

#[instrument(skip(redis_connection, user_id, items_to_add))]
pub async fn save_cart(redis_connection: &mut deadpool_redis::Connection, user_id: &Uuid, items_to_add: Vec<(String, i32)>) -> Result<(), CartError> {
    let cart_key: String = format!("cart:{}", user_id);
    const THREE_DAYS_IN_SECONDS: i64 = 259200;

    redis::pipe()
        .atomic() // to bundle the two commands, to prevent if the server crashed while executing one of them
        .hset_multiple(&cart_key, &items_to_add)
        .expire(&cart_key, THREE_DAYS_IN_SECONDS)
        .ignore() // to ignore the return value of the commands
        .query_async::<()>(redis_connection)
        .await
        .map_err(|error_message| {
           error!(error = ?error_message, "Error occurred while saving cart");
           CartError::RedisKeyError
        })

}

