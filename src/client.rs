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

    /// Get the value of key.
    pub fn get<K, V>(&mut self, key: K) -> Result<V>
    where
        K: Serialization,
        V: Deserialization,
    {
        // TODO: use Result<Option<>>
        let mut cmd = Command::new("GET");
        cmd.arg(key);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <V>::deserialization(reply)
    }

    /// Returns the bit value at offset in the string value stored at key.
    pub fn getbit<K>(&mut self, key: K, offset: i64) -> Result<u8>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("GETBIT");
        cmd.arg(key);
        cmd.arg(offset);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <u8>::deserialization(reply)
    }

    /// Returns the substring of the string value stored at key, determined by the offsets start and end (both are inclusive).
    pub fn getrange<K>(&mut self, key: K, start: i64, end: i64) -> Result<String>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("GETRANGE");
        cmd.arg(key).arg(start).arg(end);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        String::deserialization(reply)
    }

    /// Atomically sets key to value and returns the old value stored at key.
    pub fn getset<K, V>(&mut self, key: K, value: V) -> Result<String>
    where
        K: Serialization,
        V: ToString,
    {
        // TODO: use Result<Option<>>
        let mut cmd = Command::new("GETSET");
        cmd.arg(key).arg(value.to_string());

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        String::deserialization(reply)
    }

    /// Increments the number stored at key by one.
    pub fn incr<K>(&mut self, key: K) -> Result<i64>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("INCR");
        cmd.arg(key);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        i64::deserialization(reply)
    }

    /// Increments the number stored at key by increment.
    pub fn incrby<K>(&mut self, key: K, increment: i64) -> Result<i64>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("INCRBY");
        cmd.arg(key).arg(increment);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        i64::deserialization(reply)
    }

    /// Increment the string representing a floating point number stored at key by the specified increment.
    pub fn incrbyfloat<K>(&mut self, key: K, increment: f64) -> Result<f64>
    where
        K: Serialization,
    {
        let mut cmd = Command::new("INCRBYFLOAT");
        cmd.arg(key).arg(increment);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        f64::deserialization(reply)
    }

    /// Returns the values of all specified keys.
    pub fn mget<K, V>(&mut self, keys: Vec<K>) -> Result<Vec<V>>
    where
        K: Serialization,
        V: Deserialization,
    {
        let mut cmd = Command::new("MGET");
        for key in keys {
            cmd.arg(key);
        }

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);

        <Vec<V>>::deserialization(reply)
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
}
