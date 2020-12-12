use crate::pool::ConnectionPool;
use crate::redis_serialization_protocol::RedisSerializationProtocol;
use crate::Result;

pub struct Command {
    data: Vec<u8>,
}

impl Command {
    pub fn new<T: RedisSerializationProtocol>(cmd: T) -> Command {
        let mut data = Vec::new();
        data.extend_from_slice(cmd.serialization().as_slice());
        Command { data }
    }

    pub fn arg<T: RedisSerializationProtocol>(&mut self, arg: T) -> &mut Self {
        self.data.extend_from_slice(arg.serialization().as_slice());
        self
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}

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

        let cmd = Command::new("PING");
        conn.send(cmd)?;
        let reply = conn.receive()?;
        println!("reply -> {:?}", reply);

        self.pool.put(conn);

        Ok(())
    }

    pub fn set(key: String, value: String, ex: u64, px: u64, nx: bool, xx: bool) {
        let mut cmd = Command::new("SET");
        cmd.arg(key).arg(value);
        if ex > 0 {
            cmd.arg(ex);
        }
        if px > 0 {
            cmd.arg(px);
        }
    }
}
