use std::env;
use std::time::Duration;
use sqlx::{Pool, Postgres};
use crate::models::auth::AuthConfig;

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env"),

            max_connections: env::var("DB_MAX_CONNECTIONS").expect("DB_MAX_CONNECTIONS is not set in .env").parse().unwrap(),

            min_connections: env::var("DB_MIN_CONNECTIONS").expect("DB_MIN_CONNECTIONS is not set in .env").parse().unwrap(),

            acquire_timeout: Duration::from_secs(env::var("DB_ACQUIRE_TIMEOUT").expect("DB_ACQUIRE_TIMEOUT is not set in .env").parse().unwrap()),

            idle_timeout: Duration::from_secs(env::var("DB_IDLE_TIMEOUT").expect("DB_IDLE_TIMEOUT is not set in .env").parse().unwrap()),
        }
    }
}

pub struct RedisConfig {
    pub redis_url: String,
    pub max_connections: u32,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            redis_url: env::var("REDIS_URL").expect("REDIS_URL is not set in .env"),

            max_connections: env::var("REDIS_MAX_CONNECTIONS").expect("REDIS_MAX_CONNECTIONS is not set in .env").parse().unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct  AppState {
    pub db_pool: Pool<Postgres>,
    pub redis_pool: deadpool_redis::Pool,
    pub auth_config: AuthConfig
}