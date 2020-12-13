use crate::pool::ConnectionPool;
use crate::protocol::RedisSerializationProtocol;
use crate::Result;

pub struct Command {
    cmd: String,
    args: Vec<u8>,
    count: usize,
}

impl Command {
    pub fn new(cmd: String) -> Command {
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

impl RedisClient {
    pub fn new() -> RedisClient {
        RedisClient {
            pool: ConnectionPool::new(16),
        }
    }

    pub fn ping(&mut self) -> Result<()> {
        let mut conn = self.pool.get()?;
        let cmd = Command::new(String::from("PING"));
        conn.execute(cmd)?;
        self.pool.put(conn);
        Ok(())
    }

    pub fn set(&mut self, key: String, value: String, ex: u64, px: u64, nx: bool, xx: bool) -> Result<()> {
        let mut cmd = Command::new(String::from("SET"));
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
        conn.execute(cmd)?;

        Ok(())
    }

    pub fn get<T, U>(&mut self, key: T) -> Result<U>
    where
        T: RedisSerializationProtocol,
        U: RedisSerializationProtocol,
    {
        let mut cmd = Command::new(String::from("GET"));
        cmd.arg(key);

        let mut conn = self.pool.get()?;
        let reply = conn.execute(cmd)?;
        self.pool.put(conn);
        U::deserialization(reply)
    }
}
