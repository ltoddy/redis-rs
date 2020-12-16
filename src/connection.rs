use std::io::{BufRead, BufReader, Read, Write};
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
            Self::SINGLE_STRINGS => Reply::SingleStrings(Vec::from(&buffer[1..])),
            Self::ERRORS => Reply::Errors(Vec::from(&buffer[1..])),
            Self::INTEGERS => Reply::Integers(Vec::from(&buffer[1..])),
            Self::BULK_STRINGS => {
                Reply::BulkStrings(self.read_bulk(String::from_utf8_lossy(&buffer[1..]).parse::<i64>()?)?)
            }
            Self::ARRAYS => Reply::Arrays(self.read_array(String::from_utf8_lossy(&buffer[1..]).parse::<u64>()?)?),
            _ => unreachable!(),
        };

        Ok(reply)
    }

    fn read_bulk(&mut self, size: i64) -> Result<Vec<u8>> {
        if size < 0 {
            // return -1 when 'GET' an not exist key, raw data is: "$-1"
            return Err(Error::KeyNotFound);
        }

        let mut buf = vec![0; (size + 2) as usize];
        self.reader.read_exact(&mut buf)?;
        buf.truncate(buf.len() - 2);
        Ok(buf)
    }

    fn read_array(&mut self, len: u64) -> Result<Vec<Reply>> {
        let mut result = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let mut buf = Vec::new();
            self.reader.read_until(b'\n', &mut buf)?;
            let size = String::from_utf8_lossy(&buf[1..(buf.len() - 2)]).parse::<i64>()?;
            let v = self.read_bulk(size)?;
            result.push(Reply::BulkStrings(v));
        }

        Ok(result)
    }
}

#[derive(Debug)]
pub enum Reply {
    SingleStrings(Vec<u8>),
    Errors(Vec<u8>),
    Integers(Vec<u8>),
    BulkStrings(Vec<u8>),
    Arrays(Vec<Reply>),
}
