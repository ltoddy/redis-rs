use crate::error::Error;

pub mod connection;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;
