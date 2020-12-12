use crate::error::Error;

pub mod client;
pub mod connection;
pub mod error;
pub mod pool;
pub mod redis_serialization_protocol;

pub type Result<T> = std::result::Result<T, Error>;
