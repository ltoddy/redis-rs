use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::client::Command;
use crate::error::Error;
use crate::Result;

pub struct Connection {
    pub conn: TcpStream,
    pub reader: BufReader<TcpStream>,
}

impl Connection {
    const SINGLE_STRINGS: u8 = b'+';
    const ERRORS: u8 = b'-';
    const INTEGERS: u8 = b':';
    const BULK_STRINGS: u8 = b'$';
    const ARRAYS: u8 = b'*';

    pub fn new(stream: TcpStream) -> Result<Connection> {
        let reader = BufReader::new(stream.try_clone()?);

        Ok(Connection { conn: stream, reader })
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Connection> {
        let stream = TcpStream::connect(addr)?;

        Self::new(stream)
    }

    pub fn execute(&mut self, cmd: Command) -> Result<Reply> {
        self.send(cmd)?;
        self.receive()
    }

    fn send(&mut self, cmd: Command) -> Result<()> {
        let send_data = cmd.to_vec();
        self.conn.write_all(&send_data)?;
        Ok(())
    }

    fn receive(&mut self) -> Result<Reply> {
        let mut buffer = Vec::new();
        self.reader.read_until(b'\n', &mut buffer)?;
        if buffer.is_empty() {
            return Err(Error::Resp);
        }
        let buffer = &buffer[0..buffer.len() - 2];

        let reply = match buffer[0] {
            Self::SINGLE_STRINGS => Reply::SingleStrings(String::from_utf8_lossy(&buffer[1..]).to_string()),
            Self::ERRORS => Reply::Errors(String::from_utf8_lossy(&buffer[1..]).to_string()),
            Self::INTEGERS => todo!(),
            Self::BULK_STRINGS => todo!(),
            Self::ARRAYS => todo!(),

            _ => unreachable!(),
        };

        Ok(reply)
    }
}

#[derive(Debug)]
pub enum Reply {
    SingleStrings(String),
    Errors(String),
    Integers(i64),
    BulkStrings(Vec<String>),
    Arrays(Vec<Reply>),
}
