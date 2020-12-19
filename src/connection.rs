use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::error::ErrorKind::FromServer;
use crate::error::{ErrorKind::ResponseError, RedisError};
use crate::RedisResult;

pub(super) struct Connection {
    conn: TcpStream,
    reader: BufReader<TcpStream>,
}

impl Connection {
    const SINGLE_STRINGS: u8 = b'+';
    const ERRORS: u8 = b'-';
    const INTEGERS: u8 = b':';
    const BULK_STRINGS: u8 = b'$';
    const ARRAYS: u8 = b'*';
    const OK: &'static [u8] = &[79, 75, 13, 10];
    const NIL: &'static [u8] = &[36, 45, 49, 13, 10];

    pub(super) fn new(stream: TcpStream) -> RedisResult<Connection> {
        let reader = BufReader::new(stream.try_clone()?);

        Ok(Connection { conn: stream, reader })
    }

    pub(super) fn connect<A: ToSocketAddrs>(addr: A) -> RedisResult<Connection> {
        let stream = TcpStream::connect(addr)?;

        Self::new(stream)
    }

    pub(super) fn send(&mut self, data: &[u8]) -> RedisResult<()> {
        self.conn.write_all(data)?;
        Ok(())
    }

    pub(super) fn receive(&mut self) -> RedisResult<Reply> {
        let mut buffer = Vec::new();
        self.reader.read_until(b'\n', &mut buffer)?;
        if buffer.len() < 3 {
            return Err(RedisError::custom(ResponseError, "Empty redis response"));
        }
        if buffer == Self::NIL {
            // TODO: remove
            return Ok(Reply::Nil);
        }

        let prefix = buffer[0];
        let buffer = &buffer[1..buffer.len() - 2]; // remove prefix and '\r\n'

        match prefix {
            Self::SINGLE_STRINGS => self.read_single_strings(Vec::from(buffer)),
            Self::ERRORS => self.read_errors(Vec::from(buffer)),
            Self::INTEGERS => self.read_integer(Vec::from(buffer)),
            Self::BULK_STRINGS => self.read_bulk_strings(String::from_utf8_lossy(buffer).parse::<i64>()?),
            Self::ARRAYS => self.read_array(String::from_utf8_lossy(buffer).parse::<u64>()?),
            _ => Err(RedisError::custom(
                ResponseError,
                format!("invalid prefix {:?}", prefix as char),
            )),
        }
    }

    fn read_single_strings(&mut self, buffer: Vec<u8>) -> RedisResult<Reply> {
        // TODO
        if buffer == Self::OK {
            return Ok(Reply::SingleStrings(SingleStrings::Okay));
        }
        Ok(Reply::SingleStrings(SingleStrings::Okay))
    }

    fn read_errors(&mut self, buffer: Vec<u8>) -> RedisResult<Reply> {
        Err(RedisError::custom(FromServer, String::from_utf8(buffer)?))
    }

    fn read_integer(&mut self, buffer: Vec<u8>) -> RedisResult<Reply> {
        Ok(Reply::Integers(buffer))
    }

    fn read_bulk_strings(&mut self, size: i64) -> RedisResult<Reply> {
        if size < 0 {
            // TODO
            return Ok(Reply::Nil);
        }

        let mut buf = vec![0; (size + 2) as usize];
        self.reader.read_exact(&mut buf)?;
        buf.truncate(buf.len() - 2);
        Ok(Reply::BulkStrings(buf))
    }

    fn read_array(&mut self, len: u64) -> RedisResult<Reply> {
        let mut result = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let mut buf = Vec::new();
            self.reader.read_until(b'\n', &mut buf)?;
            let size = String::from_utf8_lossy(&buf[1..(buf.len() - 2)]).parse::<i64>()?;
            let v = self.read_bulk_strings(size)?;
            result.push(v);
        }

        Ok(Reply::Arrays(result))
    }
}

#[derive(Debug)]
pub enum SingleStrings {
    Okay,
}

#[derive(Debug)]
pub enum Reply {
    SingleStrings(SingleStrings),
    Integers(Vec<u8>),
    BulkStrings(Vec<u8>),
    Arrays(Vec<Reply>),
    Nil,
}
