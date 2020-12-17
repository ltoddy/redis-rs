pub mod client;
pub mod connection;
pub mod error;
pub mod pool;
pub mod protocol;

use crate::error::RedisError;

pub type RedisResult<T> = std::result::Result<T, RedisError>;
