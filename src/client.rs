use crate::config::RedisConfig;
use crate::connection::Reply;
use crate::error::{ErrorKind, RedisError};
use crate::pool::ConnectionPool;
use crate::protocol::{RedisDeserializationProtocol, RedisSerializationProtocol};
use crate::RedisResult;

struct Command {
    cmd: String,
    args: Vec<u8>,
    count: usize,
}

impl Command {
    fn new<S: ToString>(cmd: S) -> Command {
        let cmd = cmd.to_string();
        let args = Vec::new();
        Command { cmd, args, count: 1 }
    }

    fn arg<T: RedisSerializationProtocol>(&mut self, arg: T) -> &mut Self {
        self.args.extend(arg.serialization());
        self.count += 1;
        self
    }

    fn into_vec(self) -> Vec<u8> {
        let Command { cmd, args, count } = self;

        let mut buf = Vec::new();
        buf.extend(Vec::from(format!("*{}\r\n", count)));
        buf.extend(cmd.serialization());
        buf.extend(args);
        buf
    }
}

pub struct RedisClient {
    pool: ConnectionPool,
}

impl RedisClient {
    pub fn new() -> RedisResult<RedisClient> {
        let config = RedisConfig::default();

        Self::with_config(config)
    }

    pub fn with_config(config: RedisConfig) -> RedisResult<RedisClient> {
        let RedisConfig {
            address,
            database,
            username,
            password,
            pool_capacity,
        } = config;

        let mut client = RedisClient {
            pool: ConnectionPool::new(pool_capacity, address),
        };

        if let Some(password) = password {
            client.auth(username, password)?;
        }

        if database > 0 {
            client.select(database)?;
        }

        Ok(client)
    }

    // TODO
    pub fn flushall(&mut self) -> RedisResult<()> {
        let cmd = Command::new("FLUSHALL");

        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    // Connection commands
    /// The AUTH command authenticates the current connection
    ///
    /// Return value: Simple string reply
    pub fn auth<S>(&mut self, username: Option<S>, password: S) -> RedisResult<()>
    where
        S: ToString,
    {
        let mut cmd = Command::new("AUTH");
        if let Some(username) = username {
            cmd.arg(username.to_string());
        }
        cmd.arg(password.to_string());

        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    /// Returns message.
    ///
    /// Return value: Bulk string reply
    pub fn echo<S>(&mut self, message: S) -> RedisResult<String>
    where
        S: ToString,
    {
        let mut cmd = Command::new("ECHO");
        cmd.arg(message.to_string());

        let reply = self.execute(cmd)?;
        <String>::deserialization(reply)
    }

    /// Returns PONG if no argument is provided, otherwise return a copy of the argument as a bulk.
    ///
    /// Return value: Simple string reply
    pub fn ping(&mut self) -> RedisResult<()> {
        let cmd = Command::new("PING");
        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    /// Ask the server to close the connection.
    ///
    /// Return value: Simple string reply
    pub fn quit(&mut self) -> RedisResult<()> {
        let cmd = Command::new("QUIT");
        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    /// Select the Redis logical database having the specified zero-based numeric index.
    ///
    /// Return value: Simple string reply
    pub fn select(&mut self, index: u8) -> RedisResult<()> {
        let mut cmd = Command::new("SELECT");
        cmd.arg(index);

        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    // Hashes commands
    /// Removes the specified fields from the hash stored at key.
    ///
    /// Return value: Integer reply
    pub fn hdel<K>(&mut self, key: K, fields: Vec<K>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HDEL");
        cmd.arg(key);
        for field in fields {
            cmd.arg(field);
        }

        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns if field is an existing field in the hash stored at key.
    ///
    /// Return value: Integer reply
    pub fn hexists<K, F>(&mut self, key: K, field: F) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HEXISTS");
        cmd.arg(key).arg(field);

        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// Returns the value associated with field in the hash stored at key.
    ///
    /// Return value: Bulk string reply
    pub fn hget<K, F, V>(&mut self, key: K, field: F) -> RedisResult<V>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let mut cmd = Command::new("HGET");
        cmd.arg(key).arg(field);

        let reply = self.execute(cmd)?;
        <V>::deserialization(reply)
    }

    /// Returns all fields and values of the hash stored at key.
    ///
    /// Return value: Array reply
    pub fn hgetall<K, M>(&mut self, key: K) -> RedisResult<M>
    where
        K: RedisSerializationProtocol,
        M: RedisDeserializationProtocol,
    {
        let mut cmd = Command::new("HGETALL");
        cmd.arg(key);

        let reply = self.execute(cmd)?;
        <M>::deserialization(reply)
    }

    /// Increments the number stored at field in the hash stored at key by increment.
    ///
    /// Return value: Integer value
    pub fn hincrby<K, F>(&mut self, key: K, field: F, increment: i64) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HINCRBY");
        cmd.arg(key).arg(field).arg(increment);

        let reply = self.execute(cmd)?;
        <i64>::deserialization(reply)
    }

    /// Increment the specified field of a hash stored at key, and representing a floating point number, by the specified increment.
    ///
    /// Return value: Bulk string reply
    pub fn hincrbyfloat<K, F>(&mut self, key: K, field: F, increment: f64) -> RedisResult<f64>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HINCRBYFLOAT");
        cmd.arg(key).arg(field).arg(increment);

        let reply = self.execute(cmd)?;
        <f64>::deserialization(reply)
    }

    /// Returns all field names in the hash stored at key.
    ///
    /// Return value: Array reply
    pub fn hkeys<K, V>(&mut self, key: K) -> RedisResult<Vec<V>>
    where
        K: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let mut cmd = Command::new("HKEYS");
        cmd.arg(key);

        let reply = self.execute(cmd)?;
        <Vec<V>>::deserialization(reply)
    }

    /// Returns the number of fields contained in the hash stored at key.
    ///
    /// Return value: Integer reply
    pub fn hlen<K>(&mut self, key: K) -> RedisResult<u64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HLEN");
        cmd.arg(key);

        let reply = self.execute(cmd)?;
        <u64>::deserialization(reply)
    }

    /// Returns the values associated with the specified fields in the hash stored at key.
    ///
    /// Return value: Array reply
    pub fn hmget<K, F, V>(&mut self, key: K, fields: Vec<F>) -> RedisResult<Vec<V>>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let mut cmd = Command::new("HMGET");
        cmd.arg(key);
        for field in fields {
            cmd.arg(field);
        }

        let reply = self.execute(cmd)?;
        <Vec<V>>::deserialization(reply)
    }

    /// Sets the specified fields to their respective values in the hash stored at key.
    ///
    /// Return values: Simple string reply
    pub fn hmset<K, F, V>(&mut self, key: K, fvs: Vec<(F, V)>) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HMSET");
        cmd.arg(key);
        for (field, value) in fvs {
            cmd.arg(field).arg(value);
        }

        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    pub fn hscan(&mut self) {
        todo!();
    }

    /// Sets field in the hash stored at key to value.
    ///
    /// Return value: Integer reply
    pub fn hset<K, F, V>(&mut self, key: K, field: F, value: V) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HSET");
        cmd.arg(key).arg(field).arg(value);

        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Sets field in the hash stored at key to value, only if field does not yet exist.
    ///
    /// Return value: Integer value
    pub fn hsetnx<K, F, V>(&mut self, key: K, field: F, value: V) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HSETNX");
        cmd.arg(key).arg(field).arg(value);

        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns the string length of the value associated with field in the hash stored at key.
    ///
    /// Return value: Integer reply
    pub fn hstrlen<K, F>(&mut self, key: K, field: F) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        F: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("HSTRLEN");
        cmd.arg(key).arg(field);

        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns all values in the hash stored at key.
    ///
    /// Return value: Array reply
    pub fn hvals<K, V>(&mut self, key: K) -> RedisResult<Vec<V>>
    where
        K: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let mut cmd = Command::new("HVALS");
        cmd.arg(key);

        let reply = self.execute(cmd)?;
        <Vec<V>>::deserialization(reply)
    }

    // Strings commands
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

    /// Set key to hold string value if key does not exist.
    pub fn setnx<K, V>(&mut self, key: K, value: V) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("SETNX");
        cmd.arg(key).arg(value);

        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// Overwrites part of the string stored at key, starting at the specified offset, for the entire length of value.
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
        conn.send(&cmd.into_vec())?;
        let reply = conn.receive()?;
        self.pool.put(conn);
        Ok(reply)
    }
}
