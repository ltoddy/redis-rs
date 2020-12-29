use std::collections::HashSet;
use std::hash::Hash;

use crate::config::RedisConfig;
use crate::connection::Reply;
use crate::error::{ErrorKind, RedisError};
use crate::pipeline::Pipeline;
use crate::pool::ConnectionPool;
use crate::protocol::{RedisDeserializationProtocol, RedisSerializationProtocol};
use crate::{DataType, RedisResult};

pub struct Command {
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

macro_rules! command {
    ($name: expr; args => $($args: expr),*) => {
        {
            let mut cmd = Command::new($name);
            $(cmd.arg($args);)*
            cmd
        }
    };
}

// TODO: remove
pub enum ListBeforeOrAfter {
    Before,
    After,
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

    pub fn pipe(&self) -> Pipeline {
        Pipeline::new(self)
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
        let cmd = command!("ECHO"; args => message.to_string());
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
        let cmd = command!("SELECT"; args => index);
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
        let mut cmd = command!("HDEL"; args => key);
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
        let cmd = command!("HEXISTS"; args => key, field);

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
        let cmd = command!("HGET"; args => key, field);
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
        let cmd = command!("HGETALL"; args => key);
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
        let cmd = command!("HINCRBY"; args => key, field, increment);
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
        let cmd = command!("HINCRBYFLOAT"; args => key, field, increment);
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
        let cmd = command!("HKEYS"; args => key);
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
        let cmd = command!("HLEN"; args => key);
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
        let mut cmd = command!("HMGET"; args => key);
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
        let mut cmd = command!("HMSET"; args => key);
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
        let cmd = command!("HSET"; args => key, field, value);
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
        let cmd = command!("HSETNX"; args => key, field, value);
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
        let cmd = command!("HSTRLEN"; args => key, field);
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
        let cmd = command!("HVALS"; args => key);
        let reply = self.execute(cmd)?;
        <Vec<V>>::deserialization(reply)
    }

    // keys command
    /// This command copies the value stored at the source key to the destination key.
    ///
    /// Return value: Integer reply
    pub fn copy(&mut self) -> RedisResult<()> {
        todo!()
    }

    /// Removes the specified keys. A key is ignored if it does not exist.
    ///
    /// Return value: Integer reply
    pub fn del<K>(&mut self, keys: Vec<K>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("DEL");
        for key in keys {
            cmd.arg(key);
        }

        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Serialize the value stored at key in a Redis-specific format and return it to the user.
    ///
    /// Return value: Bulk string reply
    #[allow(unused_variables)]
    pub fn dump<K>(&mut self, key: K) -> RedisResult<String>
    where
        K: RedisSerializationProtocol,
    {
        // let mut cmd = Command::new("DUMP");
        // cmd.arg(key);
        //
        // let reply = self.execute(cmd)?;
        // <String>::deserialization(reply)
        todo!()
    }

    /// Returns if key exists.
    ///
    /// Return value: Integer reply
    pub fn exists<K>(&mut self, keys: Vec<K>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("EXISTS");
        for key in keys {
            cmd.arg(key);
        }

        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Set a timeout on key. After the timeout has expired, the key will automatically be deleted.
    ///
    /// Return value: Integer reply
    pub fn expire<K>(&mut self, key: K, seconds: usize) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("EXPIRE"; args => key, seconds);
        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// EXPIREAT has the same effect and semantic as EXPIRE, but instead of specifying the number of seconds representing the TTL, it takes an absolute Unix timestamp.
    ///
    /// Return value: Integer reply
    #[allow(unused_variables)]
    pub fn expireat<K>(&mut self, key: K, timestamp: u64) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
    {
        todo!()
    }

    /// Returns all keys matching pattern.
    ///
    /// Return value: Array reply
    pub fn keys<S>(&mut self, pattern: S) -> RedisResult<Vec<String>>
    where
        S: ToString,
    {
        let cmd = command!("KEYS"; args => pattern.to_string());
        let reply = self.execute(cmd)?;
        <Vec<String>>::deserialization(reply)
    }

    /// Remove the existing timeout on key, turning the key from volatile to persistent.
    ///
    /// Return value: Integer reply
    pub fn persist<K>(&mut self, key: K) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("PERSIST"; args => key);
        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// This command works exactly like EXPIRE but the time to live of the key is specified in milliseconds instead of seconds.
    ///
    /// Return value: Integer reply
    pub fn pexpire<K>(&mut self, key: K, milliseconds: u64) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("PEXPIRE"; args => key, milliseconds);
        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// Like TTL this command returns the remaining time to live of a key that has an expire set,
    /// with the sole difference that TTL returns the amount of remaining time in seconds while PTTL returns it in milliseconds.
    ///
    /// Return value: Integer reply
    pub fn pttl<K>(&mut self, key: K) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("PTTL"; args => key);
        let reply = self.execute(cmd)?;
        <i64>::deserialization(reply)
    }

    /// Return a random key from the currently selected database.
    ///
    /// Return value: Bulk string reply
    pub fn randomkey(&mut self) -> RedisResult<String> {
        let cmd = Command::new("RANDOMKEY");
        let reply = self.execute(cmd)?;
        <String>::deserialization(reply)
    }

    /// Renames key to newkey. It returns an error when key does not exist.
    ///
    /// Return value: Simple string reply
    pub fn rename<K>(&mut self, key: K, newkey: K) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("RENAME"; args => key, newkey);
        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    /// Renames key to newkey if newkey does not yet exist. It returns an error when key does not exist.
    ///
    /// Return value: Integer reply
    pub fn renamenx<K>(&mut self, key: K, newkey: K) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("RENAMENX"; args => key, newkey);
        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// Alters the last access time of a key(s). A key is ignored if it does not exist.
    ///
    /// Return value: Integer reply
    pub fn touch<K>(&mut self, keys: Vec<K>) -> RedisResult<isize>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("TOUCH");
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <isize>::deserialization(reply)
    }

    /// Returns the remaining time to live of a key that has a timeout.
    ///
    /// Return value: Integer reply
    pub fn ttl<K>(&mut self, key: K) -> RedisResult<isize>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("TTL"; args => key);
        let reply = self.execute(cmd)?;
        <isize>::deserialization(reply)
    }

    /// Returns the string representation of the type of the value stored at key.
    ///
    /// Return value: Simple string reply
    pub fn type_<K>(&mut self, key: K) -> RedisResult<DataType>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("TYPE"; args => key);
        let reply = self.execute(cmd)?;
        <DataType>::deserialization(reply)
    }

    /// This command is very similar to DEL: it removes the specified keys.
    ///
    /// Return value: Integer reply
    pub fn unlink<K>(&mut self, keys: Vec<K>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = Command::new("UNLINK");
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// BRPOPLPUSH is the blocking variant of RPOPLPUSH.
    ///
    /// Return value: Bulk string reply
    pub fn brpoplpush<K, E>(&mut self, source: K, destination: K, timeout: usize) -> RedisResult<E>
    where
        K: RedisSerializationProtocol,
        E: RedisDeserializationProtocol,
    {
        let cmd = command!("BRPOPLPUSH"; args => source, destination, timeout);
        let reply = self.execute(cmd)?;
        <E>::deserialization(reply)
    }

    // Lists commands
    /// Returns the element at index index in the list stored at key.
    ///
    /// Return value: Bulk string reply
    pub fn lindex<K, V>(&mut self, key: K, index: isize) -> RedisResult<V>
    where
        K: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let cmd = command!("LINDEX"; args => key, index);
        let reply = self.execute(cmd)?;
        <V>::deserialization(reply)
    }

    /// Inserts element in the list stored at key either before or after the reference value pivot.
    ///
    /// Return value: Integer reply
    pub fn linsert<K, E>(&mut self, key: K, operator: ListBeforeOrAfter, pivot: E, element: E) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        E: RedisSerializationProtocol,
    {
        let cmd = command!("LINSERT"; args => key, operator, pivot, element);
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns the length of the list stored at key.
    ///
    /// Return value: Integer reply
    pub fn llen<K>(&mut self, key: K) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("LLEN"; args => key);
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Removes and returns the first element of the list stored at key.
    ///
    /// Return value: Bulk string reply
    pub fn lpop<K, E>(&mut self, key: K) -> RedisResult<E>
    where
        K: RedisSerializationProtocol,
        E: RedisDeserializationProtocol,
    {
        let cmd = command!("LPOP"; args => key);
        let reply = self.execute(cmd)?;
        <E>::deserialization(reply)
    }

    /// The command returns the index of matching elements inside a Redis list.
    pub fn lpos(&mut self) {
        todo!()
    }

    /// Insert all the specified values at the head of the list stored at key.
    ///
    /// Retrun value: Integer reply
    pub fn lpush<K, E>(&mut self, key: K, elements: Vec<E>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        E: RedisSerializationProtocol,
    {
        let mut cmd = command!("LPUSH"; args => key);
        for element in elements {
            cmd.arg(element);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Inserts specified values at the head of the list stored at key, only if key already exists and holds a list.
    ///
    /// Return value: Integer value
    pub fn lpushx<K, E>(&mut self, key: K, elements: Vec<E>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        E: RedisSerializationProtocol,
    {
        let mut cmd = command!("LPUSHX"; args => key);
        for element in elements {
            cmd.arg(element);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns the specified elements of the list stored at key.
    ///
    /// Return value: Array reply
    pub fn lrange<K, E>(&mut self, key: K, start: isize, end: isize) -> RedisResult<Vec<E>>
    where
        K: RedisSerializationProtocol,
        E: RedisDeserializationProtocol,
    {
        let cmd = command!("LRANGE"; args => key, start, end);
        let reply = self.execute(cmd)?;
        <Vec<E>>::deserialization(reply)
    }

    /// Removes the first count occurrences of elements equal to element from the list stored at key.
    ///
    /// Return value: Integer reply
    pub fn lrem<K, E>(&mut self, key: K, count: isize, element: E) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        E: RedisSerializationProtocol,
    {
        let cmd = command!("LREM"; args => key, count, element);
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Sets the list element at index to element.
    ///
    /// Return value: Simple string reply
    pub fn lset<K, E>(&mut self, key: K, index: isize, element: E) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        E: RedisSerializationProtocol,
    {
        let cmd = command!("LSET"; args => key, index, element);
        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    /// Trim an existing list so that it will contain only the specified range of elements specified.
    ///
    /// Return value: Simple string reply
    pub fn ltrim<K>(&mut self, key: K, start: isize, stop: isize) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("LTRIM"; args => key, start, stop);
        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    /// Removes and returns the last element of the list stored at key.
    ///
    /// Return value: Bulk string reply
    pub fn rpop<K, E>(&mut self, key: K) -> RedisResult<E>
    where
        K: RedisSerializationProtocol,
        E: RedisDeserializationProtocol,
    {
        let cmd = command!("RPOP"; args => key);
        let reply = self.execute(cmd)?;
        <E>::deserialization(reply)
    }

    /// Atomically returns and removes the last element (tail) of the list stored at source, and
    /// pushes the element at the first element (head) of the list stored at destination.
    ///
    /// Return value: Bulk string reply
    pub fn rpoplpush<K, E>(&mut self, source: K, destination: K) -> RedisResult<E>
    where
        K: RedisSerializationProtocol,
        E: RedisDeserializationProtocol,
    {
        let cmd = command!("RPOPLPUSH"; args => source, destination);
        let reply = self.execute(cmd)?;
        <E>::deserialization(reply)
    }

    /// Insert all the specified values at the tail of the list stored at key.
    ///
    /// Return value: Integer value
    pub fn rpush<K, E>(&mut self, key: K, elements: Vec<E>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        E: RedisSerializationProtocol,
    {
        let mut cmd = command!("RPUSH"; args => key);
        for element in elements {
            cmd.arg(element);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Inserts specified values at the tail of the list stored at key, only if key already exists and holds a list.
    ///
    /// Return value: Integer reply
    pub fn rpushx<K, E>(&mut self, key: K, elements: Vec<E>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        E: RedisSerializationProtocol,
    {
        let mut cmd = command!("RPUSHX"; args => key);
        for element in elements {
            cmd.arg(element);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    // Sets commands
    /// Add the specified members to the set stored at key.
    ///
    /// Return value: Integer value
    pub fn sadd<K, M>(&mut self, key: K, members: HashSet<M>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        M: RedisSerializationProtocol + Hash + Eq,
    {
        let mut cmd = command!("SADD"; args => key);
        for member in members {
            cmd.arg(member);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns the set cardinality (number of elements) of the set stored at key.
    ///
    /// Return value: Integer reply
    pub fn scard<K>(&mut self, key: K) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("SCARD"; args => key);
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns the members of the set resulting from the difference between the first set and all the successive sets.
    ///
    /// Return value: Array reply
    pub fn sdiff<K, M>(&mut self, keys: Vec<K>) -> RedisResult<HashSet<M>>
    where
        K: RedisSerializationProtocol,
        M: RedisDeserializationProtocol + Hash + Eq,
    {
        let mut cmd = Command::new("SDIFF");
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <HashSet<M>>::deserialization(reply)
    }

    /// This command is equal to SDIFF, but instead of returning the resulting set, it is stored in destination.
    ///
    /// Return value: Integer reply
    pub fn sdiffstore<K>(&mut self, destination: K, keys: Vec<K>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol + Hash + Eq,
    {
        let mut cmd = command!("SDIFFSTORE"; args => destination);
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns the members of the set resulting from the intersection of all the given sets.
    ///
    /// Return value: Array reply
    pub fn sinter<K, M>(&mut self, keys: Vec<K>) -> RedisResult<HashSet<M>>
    where
        K: RedisSerializationProtocol,
        M: RedisDeserializationProtocol + Hash + Eq,
    {
        let mut cmd = Command::new("SINTER");
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <HashSet<M>>::deserialization(reply)
    }

    /// This command is equal to SINTER, but instead of returning the resulting set, it is stored in destination.
    ///
    /// Return value: Integer reply
    pub fn sinterstore<K>(&mut self, destination: K, keys: Vec<K>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = command!("SINTERSTORE"; args => destination);
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns if member is a member of the set stored at key.
    ///
    /// Return value: Integer reply
    pub fn sismember<K, M>(&mut self, key: K, member: M) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
        M: RedisSerializationProtocol + Hash + Eq,
    {
        let cmd = command!("SISMEMBER"; args => key, member);
        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// Returns all the members of the set value stored at key.
    ///
    /// Return value: Array reply
    pub fn smembers<K, M>(&mut self, key: K) -> RedisResult<HashSet<M>>
    where
        K: RedisSerializationProtocol,
        M: RedisDeserializationProtocol + Hash + Eq,
    {
        let cmd = command!("SMEMBERS"; args => key);
        let reply = self.execute(cmd)?;
        <HashSet<M>>::deserialization(reply)
    }

    /// Returns whether each member is a member of the set stored at key.
    ///
    /// Return value: Array reply
    pub fn smismember<K, M>(&mut self, key: K, members: HashSet<M>) -> RedisResult<Vec<bool>>
    where
        K: RedisSerializationProtocol,
        M: RedisSerializationProtocol + Hash + Eq,
    {
        let mut cmd = command!("SMISMEMBER"; args => key);
        for member in members {
            cmd.arg(member);
        }
        let reply = self.execute(cmd)?;
        <Vec<bool>>::deserialization(reply)
    }

    /// Move member from the set at source to the set at destination.
    ///
    /// Return value: Integer reply
    pub fn smove<K, M>(&mut self, source: K, destination: K, member: M) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        M: RedisSerializationProtocol + Hash + Eq,
    {
        let cmd = command!("SMOVE"; args => source, destination, member);
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Removes and returns one or more random members from the set value store at key.
    ///
    /// Return value: Bulk string reply or Array reply
    pub fn spop<K, M>(&mut self, key: K, count: Option<usize>) -> RedisResult<M>
    where
        K: RedisSerializationProtocol,
        M: RedisDeserializationProtocol,
    {
        let mut cmd = command!("SPOP"; args => key);
        if let Some(count) = count {
            cmd.arg(count);
        }
        let reply = self.execute(cmd)?;
        <M>::deserialization(reply)
    }

    /// When called with just the key argument, return a random element from the set value stored at key.
    ///
    /// Return value: Bulk string reply or Array reply
    pub fn srandmember<K, M>(&mut self, key: K, count: Option<usize>) -> RedisResult<M>
    where
        K: RedisSerializationProtocol,
        M: RedisDeserializationProtocol,
    {
        let mut cmd = command!("SPOP"; args => key);
        if let Some(count) = count {
            cmd.arg(count);
        }
        let reply = self.execute(cmd)?;
        <M>::deserialization(reply)
    }

    /// Remove the specified members from the set stored at key.
    ///
    /// Return value: Integer reply
    pub fn srem<K, M>(&mut self, key: K, members: HashSet<M>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        M: RedisSerializationProtocol + Hash + Eq,
    {
        let mut cmd = command!("SREM"; args => key);
        for member in members {
            cmd.arg(member);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Like Scan command
    pub fn sscan() {
        todo!()
    }

    /// Returns the members of the set resulting from the union of all the given sets.
    ///
    /// Return value: Array reply
    pub fn sunion<K, M>(&mut self, keys: Vec<K>) -> RedisResult<HashSet<M>>
    where
        K: RedisSerializationProtocol,
        M: RedisDeserializationProtocol + Hash + Eq,
    {
        let mut cmd = Command::new("SUNION");
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <HashSet<M>>::deserialization(reply)
    }

    /// This command is equal to SUNION, but instead of returning the resulting set, it is stored in destination.
    ///
    /// Return value: Integer reply
    pub fn sunionstore<K>(&mut self, destination: K, keys: Vec<K>) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = command!("SUNIONSTORE"; args => destination);
        for key in keys {
            cmd.arg(key);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    // Sorted Sets commands
    // TODO: design data structure
    /// Adds all the specified members with the specified scores to the sorted set stored at key.
    ///
    /// Return value: Integer reply
    pub fn zadd<K, M>(&mut self, key: K, paris: Vec<(isize, M)>) -> RedisResult<usize>
    // TODO: refactor parameters
    where
        K: RedisSerializationProtocol,
        M: RedisSerializationProtocol,
    {
        let mut cmd = command!("ZADD"; args => key);
        for (score, member) in paris {
            cmd.arg(score).arg(member);
        }
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    // Strings commands
    pub fn append<K, V>(&mut self, key: K, value: V) -> RedisResult<u64>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let cmd = command!("APPEND"; args => key, value);
        let reply = self.execute(cmd)?;
        <u64>::deserialization(reply)
    }

    /// Count the number of set bits (population counting) in a string.
    pub fn bitcount<K>(&mut self, key: K, start: Option<i64>, end: Option<i64>) -> RedisResult<u64>
    where
        K: RedisSerializationProtocol,
    {
        let mut cmd = command!("BITCOUNT"; args => key);
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
        let mut cmd = command!("BITOP"; args => operation, destkey);
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

        let mut cmd = command!("BITPOS"; args => key, bit);
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
        let cmd = command!("DECR"; args => key);
        let reply = self.execute(cmd)?;
        <i64>::deserialization(reply)
    }

    /// Decrements the number stored at key by decrement.
    pub fn decrby<K>(&mut self, key: K, decrement: i64) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("DECRBY"; args => key, decrement);
        let reply = self.execute(cmd)?;
        <i64>::deserialization(reply)
    }

    /// Get the value of key.
    pub fn get<K, V>(&mut self, key: K) -> RedisResult<V>
    where
        K: RedisSerializationProtocol,
        V: RedisDeserializationProtocol,
    {
        let cmd = command!("GET"; args => key);
        let reply = self.execute(cmd)?;
        <V>::deserialization(reply)
    }

    /// Returns the bit value at offset in the string value stored at key.
    pub fn getbit<K>(&mut self, key: K, offset: i64) -> RedisResult<u8>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("GETBIT"; args => key, offset);
        let reply = self.execute(cmd)?;
        <u8>::deserialization(reply)
    }

    /// Returns the substring of the string value stored at key, determined by the offsets start and end (both are inclusive).
    pub fn getrange<K>(&mut self, key: K, start: i64, end: i64) -> RedisResult<String>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("GETRANGE"; args => key, start, end);
        let reply = self.execute(cmd)?;
        <String>::deserialization(reply)
    }

    /// Atomically sets key to value and returns the old value stored at key.
    pub fn getset<K, V>(&mut self, key: K, value: V) -> RedisResult<String>
    where
        K: RedisSerializationProtocol,
        V: ToString,
    {
        let cmd = command!("GETSET"; args => key, value.to_string());
        let reply = self.execute(cmd)?;
        <String>::deserialization(reply)
    }

    /// Increments the number stored at key by one.
    pub fn incr<K>(&mut self, key: K) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("INCR"; args => key);
        let reply = self.execute(cmd)?;
        <i64>::deserialization(reply)
    }

    /// Increments the number stored at key by increment.
    pub fn incrby<K>(&mut self, key: K, increment: i64) -> RedisResult<i64>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("INCRBY"; args => key, increment);
        let reply = self.execute(cmd)?;
        <i64>::deserialization(reply)
    }

    /// Increment the string representing a floating point number stored at key by the specified increment.
    pub fn incrbyfloat<K>(&mut self, key: K, increment: f64) -> RedisResult<f64>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("INCRBYFLOAT"; args => key, increment);
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
        let cmd = command!("PSETEX"; args => key, milliseconds, value);
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
        let mut cmd = command!("SET"; args => key, value);
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
        let cmd = command!("SETBIT"; args => key, offset, value);
        let reply = self.execute(cmd)?;
        <u8>::deserialization(reply)
    }

    /// Set key to hold the string value and set key to timeout after a given number of seconds.
    pub fn setex<K, V>(&mut self, key: K, seconds: usize, value: V) -> RedisResult<()>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let cmd = command!("SETEX"; args => key, seconds, value);
        let reply = self.execute(cmd)?;
        <()>::deserialization(reply)
    }

    /// Set key to hold string value if key does not exist.
    pub fn setnx<K, V>(&mut self, key: K, value: V) -> RedisResult<bool>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let cmd = command!("SETNX"; args => key, value);
        let reply = self.execute(cmd)?;
        <bool>::deserialization(reply)
    }

    /// Overwrites part of the string stored at key, starting at the specified offset, for the entire length of value.
    pub fn setrange<K, V>(&mut self, key: K, offset: usize, value: V) -> RedisResult<usize>
    where
        K: RedisSerializationProtocol,
        V: RedisSerializationProtocol,
    {
        let cmd = command!("SETRANGE"; args => key, offset, value);
        let reply = self.execute(cmd)?;
        <usize>::deserialization(reply)
    }

    /// Returns the length of the string value stored at key.
    pub fn strlen<K>(&mut self, key: K) -> RedisResult<u64>
    where
        K: RedisSerializationProtocol,
    {
        let cmd = command!("STRLEN"; args => key);
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
