use crate::connection::Reply;
use crate::error::{ErrorKind, RedisError};
use crate::pool::ConnectionPool;
use crate::protocol::{RedisDeserializationProtocol, RedisSerializationProtocol};
use crate::RedisResult;

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

    pub fn arg<T: RedisSerializationProtocol>(&mut self, arg: T) -> &mut Self {
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

    pub fn ping(&mut self) -> RedisResult<()> {
        let cmd = Command::new("PING");
        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    pub fn append<K, V>(&mut self, key: K, value: V) -> RedisResult<u64>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("APPEND");
        cmd.arg(key).arg(value);

        let reply = self.execute(cmd)?;

        <u64>::deserialization(reply)
    }

    /// Count the number of set bits (population counting) in a string.
    pub fn bitcount<K>(&mut self, key: K, start: Option<i64>, end: Option<i64>) -> RedisResult<u64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("BITCOUNT");
        cmd.arg(key);
        if let Some(start) = start {
            cmd.arg(start);
        }
        if let Some(end) = end {
            cmd.arg(end);
        }

        let reply = self.execute(cmd)?;

        <u64>::deserialization(reply)
    }

    /// Perform a bitwise operation between multiple keys (containing string values) and store the result in the destination key.
    pub fn bitop<K1, K2>(&mut self, operation: &str, destkey: K1, keys: Vec<K2>) -> RedisResult<usize>
    where
        K1: RedisSerializationProtocol,
        K2: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("BITOP");
        cmd.arg(operation).arg(destkey);
        for key in keys {
            cmd.arg(key);
        }

        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Return the position of the first bit set to 1 or 0 in a string.
    pub fn bitpos<K>(&mut self, key: K, bit: u8, start: Option<usize>, end: Option<usize>) -> RedisResult<isize>
    where
        K: RedisSerializationProtocol,
    {
        if end.is_some() && start.is_none() {
            return Err(RedisError::custom(
                ErrorKind::ClientError,
                "`start` shouldn't be none when `end` has given",
            ));
        }

        let mut cmd = Command::new("BITPOS");
        cmd.arg(key).arg(bit);
        if let Some(start) = start {
            cmd.arg(start);
        }
        if let Some(end) = end {
            cmd.arg(end);
        }

        let reply = self.execute(cmd)?;
        <isize>::deserialization(reply)
    }

    /// Decrements the number stored at key by one.
    pub fn decr<K>(&mut self, key: K) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("DECR");
        cmd.arg(key);

        let reply = self.execute(cmd)?;

        <i64>::deserialization(reply)
    }

    /// Decrements the number stored at key by decrement.
    pub fn decrby<K>(&mut self, key: K, decrement: i64) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("DECRBY");
        cmd.arg(key).arg(decrement);

        let reply = self.execute(cmd)?;

        <i64>::deserialization(reply)
    }

    /// Get the value of key.
    pub fn get<K, V>(&mut self, key: K) -> RedisResult<V>
    where
        K: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let mut cmd = Command::new("GET");
        cmd.arg(key);

        let reply = self.execute(cmd)?;

        <V>::deserialization(reply)
    }

    /// Returns the bit value at offset in the string value stored at key.
    pub fn getbit<K>(&mut self, key: K, offset: i64) -> RedisResult<u8>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("GETBIT");
        cmd.arg(key);
        cmd.arg(offset);

        let reply = self.execute(cmd)?;

        <u8>::deserialization(reply)
    }

    /// Returns the substring of the string value stored at key, determined by the offsets start and end (both are inclusive).
    pub fn getrange<K>(&mut self, key: K, start: i64, end: i64) -> RedisResult<String>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("GETRANGE");
        cmd.arg(key).arg(start).arg(end);

        let reply = self.execute(cmd)?;

        <String>::deserialization(reply)
    }

    /// Atomically sets key to value and returns the old value stored at key.
    pub fn getset<K, V>(&mut self, key: K, value: V) -> RedisResult<String>
    where
        K: RedisSerializationProtocol,
        V: ToString,
    {
        let mut cmd = Command::new("GETSET");
        cmd.arg(key).arg(value.to_string());

        let reply = self.execute(cmd)?;

        <String>::deserialization(reply)
    }

    /// Increments the number stored at key by one.
    pub fn incr<K>(&mut self, key: K) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("INCR");
        cmd.arg(key);

        let reply = self.execute(cmd)?;

        <i64>::deserialization(reply)
    }

    /// Increments the number stored at key by increment.
    pub fn incrby<K>(&mut self, key: K, increment: i64) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("INCRBY");
        cmd.arg(key).arg(increment);

        let reply = self.execute(cmd)?;

        <i64>::deserialization(reply)
    }

    /// Increment the string representing a floating point number stored at key by the specified increment.
    pub fn incrbyfloat<K>(&mut self, key: K, increment: f64) -> RedisResult<f64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("INCRBYFLOAT");
        cmd.arg(key).arg(increment);

        let reply = self.execute(cmd)?;

        <f64>::deserialization(reply)
    }

    /// Returns the values of all specified keys.
    pub fn mget<K, V>(&mut self, keys: Vec<K>) -> RedisResult<Vec<V>>
    where
        K: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let mut cmd = Command::new("MGET");
        for key in keys {
            cmd.arg(key);
        }

        let reply = self.execute(cmd)?;

        <Vec<V>>::deserialization(reply)
    }

    /// Sets the given keys to their respective values.
    pub fn mset<K, V>(&mut self, kvs: Vec<(K, V)>) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("MSET");
        for (k, v) in kvs {
            cmd.arg(k).arg(v);
        }

        let reply = self.execute(cmd)?;

        <()>::deserialization(reply)
    }

    /// Sets the given keys to their respective values.
    pub fn msetnx<K, V>(&mut self, kvs: Vec<(K, V)>) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("MSETNX");
        for (k, v) in kvs {
            cmd.arg(k).arg(v);
        }

        let reply = self.execute(cmd)?;

        <()>::deserialization(reply)
    }

    /// PSETEX works exactly like SETEX with the sole difference that the expire time is specified in milliseconds instead of seconds.
    pub fn psetex<K, V>(&mut self, key: K, milliseconds: u64, value: V) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("PSETEX");
        cmd.arg(key).arg(milliseconds).arg(value);

        let reply = self.execute(cmd)?;

        <()>::deserialization(reply)
    }

    /// Set key to hold the string value.
    pub fn set<K, V>(
        &mut self,
        key: K,
        value: V,
        ex_seconds: Option<u64>,
        px_milliseconds: Option<u64>,
        nx: Option<bool>,
        xx: Option<bool>,
    ) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("SET");
        cmd.arg(key).arg(value);
        if let Some(ex) = ex_seconds {
            cmd.arg("EX").arg(ex);
        }
        if let Some(px) = px_milliseconds {
            cmd.arg("PX").arg(px);
        }
        if let Some(nx) = nx {
            if nx {
                cmd.arg("NX");
            }
        }
        if let Some(xx) = xx {
            if xx {
                cmd.arg("XX");
            }
        }

        let reply = self.execute(cmd)?;

        <()>::deserialization(reply)
    }

    /// Set key to hold the string value.
    pub fn simple_set<K, V>(&mut self, key: K, value: V) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        self.set(key, value, None, None, None, None)
    }

    /// Sets or clears the bit at offset in the string value stored at key.
    pub fn setbit<K>(&mut self, key: K, offset: usize, value: u8) -> RedisResult<u8>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("SETBIT");
        cmd.arg(key).arg(offset).arg(value);

        let reply = self.execute(cmd)?;

        <u8>::deserialization(reply)
    }

    /// Set key to hold the string value and set key to timeout after a given number of seconds.
    pub fn setex<K, V>(&mut self, key: K, seconds: usize, value: V) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("SETEX");
        cmd.arg(key).arg(seconds).arg(value);

        let reply = self.execute(cmd)?;

        <()>::deserialization(reply)
    }

    pub fn setnx<K, V>(&mut self, key: K, value: V) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("SETNX");
        cmd.arg(key).arg(value);

        let reply = self.execute(cmd)?;
        let res = <u8>::deserialization(reply)?;

        Ok(res > 0)
    }

    pub fn setrange<K, V>(&mut self, key: K, offset: usize, value: V) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("SETRANGE");
        cmd.arg(key).arg(offset).arg(value);

        let reply = self.execute(cmd)?;

        <usize>::deserialization(reply)
    }

    /// Returns the length of the string value stored at key.
    pub fn strlen<K>(&mut self, key: K) -> RedisResult<u64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("STRLEN");
        cmd.arg(key);

        let reply = self.execute(cmd)?;

        <u64>::deserialization(reply)
    }

    fn execute(&mut self, cmd: Command) -> RedisResult<Reply> {
        let mut conn = self.pool.get()?;
        conn.send(cmd)?;
        let reply = conn.receive()?;
        Ok(reply)
    }
}
