//! ```toml
//! [dependencies.redisclient]
//! version = "*"
//! ```
//!
//! If you want use the Git version:
//!
//! ```toml
//! [dependencies.redisclient]
//! git = "https://github.com/ltoddy/redis-rs.git"
//! ```
//!
//! ### simple usage
//!
//! ```rust
//! extern crate redisclient;
//!
//! use redisclient::RedisResult;
//! use redisclient::RedisClient;
//!
//! fn run() -> RedisResult<()> {
//!     let mut client = RedisClient::new()?;
//!     client.mset(vec![("key1", 1), ("key2", 2)])?;
//!
//!     let values: Vec<String> = client.mget(vec!["key1", "key2"])?;
//!     println!("values -> {:?}", values);
//!
//!     let values: Vec<isize> = client.mget(vec!["key1", "key2"])?;
//!     println!("values -> {:?}", values);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod config;
pub mod connection;
pub mod error;
pub mod macros;
pub mod pool;
pub mod protocol;

pub use client::RedisClient;
pub use error::{ErrorKind, RedisError};

pub type RedisResult<T> = std::result::Result<T, RedisError>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataType {
    String,
    List,
    Set,
}
