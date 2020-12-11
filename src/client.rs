use crate::pool::ConnectionPool;
use crate::Result;

pub struct RedisClient {
    pool: ConnectionPool,
}

impl RedisClient {
    pub fn new() -> RedisClient {
        RedisClient {
            pool: ConnectionPool::new(16),
        }
    }

    pub fn ping(&mut self) -> Result<()> {
        let mut conn = self.pool.get()?;
        conn.send(String::from("PING"))?;
        Ok(())
    }
}
