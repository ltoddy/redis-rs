use crate::pool::ConnectionPool;
use crate::protocol::{Deserialization, Serialization};
use crate::Result;

pub struct Command {
    cmd: String,
    args: Vec<u8>,
    count: usize,
}

impl Command {
    pub fn new<S: ToString>(cmd: S) -> Command {
        let cmd = cmd.to_string();
        let args = Vec::new();
        Command { cmd, args, count: 1 }
    }

    pub fn arg<T: Serialization>(&mut self, arg: T) -> &mut Self {
        self.args.extend_from_slice(arg.serialization().as_slice());
        self.count += 1;
        self
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("*{}\r\n", self.count).as_bytes());
        buf.extend_from_slice(self.cmd.serialization().as_slice());
        buf.extend_from_slice(&self.args);
        buf
    }
}

pub struct RedisClient {
    pool: ConnectionPool,
}

impl Default for RedisClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RedisClient {
    pub fn new() -> RedisClient {
        RedisClient {
            pool: ConnectionPool::new(16),
        }
    }

    pub fn ping(&mut self) -> Result<()> {
        let cmd = Command::new("PING");

        let mut conn = self.pool.get()?;
        let _ = conn.execute(cmd)?;
        self.pool.put(conn);

        Ok(())
    }

    pub fn append<K, V>(&mut self, key: K, value: V) -> Result<u64>
    where
        K: Serialization,
        V: Serialization,
    {
        let mut cmd = Command::new("APPEND");
        cmd.arg(key).arg(value);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <u64>::deserialization(reply)
    }

    /// Count the number of set bits (population counting) in a string.
    pub fn bitcount<K>(&mut self, key: K, start: Option<i64>, end: Option<i64>) -> Result<u64>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("BITCOUNT");
        cmd.arg(key);
        if let Some(start) = start {
            cmd.arg(start);
        }
        if let Some(end) = end {
            cmd.arg(end);
        }

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <u64>::deserialization(reply)
    }

    /// Decrements the number stored at key by one.
    pub fn decr<K>(&mut self, key: K) -> Result<i64>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("DECR");
        cmd.arg(key);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <i64>::deserialization(reply)
    }

    /// Decrements the number stored at key by decrement.
    pub fn decrby<K>(&mut self, key: K, decrement: i64) -> Result<i64>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("DECRBY");
        cmd.arg(key).arg(decrement);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <i64>::deserialization(reply)
    }

    pub fn set<K, V>(&mut self, key: K, value: V, ex: u64, px: u64, nx: bool, xx: bool) -> Result<()>
    where
        K: Serialization,
        V: Serialization,
    {
        let mut cmd = Command::new("SET");
        cmd.arg(key).arg(value);
        if ex > 0 {
            cmd.arg("EX").arg(ex);
        }
        if px > 0 {
            cmd.arg("PX").arg(px);
        }
        if nx {
            cmd.arg("XX");
        } else if xx {
            cmd.arg("NX");
        }

        let mut conn = self.pool.get()?;
        let _ = conn.execute(cmd)?;
        self.pool.put(conn);

        Ok(())
    }

    pub fn get<K, V>(&mut self, key: K) -> Result<V>
    where
        K: Serialization,
        V: Deserialization,
    {
        let mut cmd = Command::new("GET");
        cmd.arg(key);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <V>::deserialization(reply)
    }
}
