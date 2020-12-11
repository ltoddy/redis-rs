use crate::error::Error;

pub mod client;
pub mod connection;
pub mod error;
pub mod pool;

pub type Result<T> = std::result::Result<T, Error>;

pub trait ToRedisArgs {
    fn to_redis_args(&self) -> Vec<u8>;
}

impl ToRedisArgs for String {
    fn to_redis_args(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(b"*1\r\n");
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(format!("{}\r\n", self).as_bytes());
        buf
    }
}
