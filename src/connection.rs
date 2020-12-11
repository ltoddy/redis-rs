use std::io::BufReader;
use std::net::{TcpStream, ToSocketAddrs};

use crate::Result;

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

    pub fn send() -> Result<()> {
        todo!()
    }

    pub fn receive() -> Result<()> {
        todo!()
    }
}
