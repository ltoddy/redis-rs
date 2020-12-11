use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::error::Error;
use crate::Result;
use crate::ToRedisArgs;

pub struct Connection {
    conn: TcpStream,
    reader: BufReader<TcpStream>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Result<Connection> {
        let reader = BufReader::new(stream.try_clone()?);

        let conn = Connection { conn: stream, reader };

        Ok(conn)
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Connection> {
        let stream = TcpStream::connect(addr)?;

        Self::new(stream)
    }

    pub fn send<Args: ToRedisArgs>(&mut self, args: Args) -> Result<()> {
        let buf = args.to_redis_args();
        self.conn.write_all(&buf)?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<Reply> {
        let mut buffer = Vec::new();
        self.reader.read_until(b'\n', &mut buffer)?;
        if buffer.is_empty() {
            return Err(Error::Resp);
        }
        let buffer = &buffer[0..buffer.len() - 2];

        let reply = match buffer[0] {
            b'+' => Reply::SingleStrings(String::from_utf8_lossy(&buffer[1..]).to_owned().to_string()),

            _ => unreachable!(),
        };

        Ok(reply)
    }
}

#[derive(Debug)]
pub enum Reply {
    SingleStrings(String),
    Errors(Error),
    Integers(i64),
    BulkStrings(Vec<String>),
    Arrays(Vec<Reply>),
}
