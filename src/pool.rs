use std::collections::VecDeque;

use crate::connection::Connection;
use crate::error::Error;
use crate::Result;

pub struct ConnectionPool {
    addr: String,
    capacity: usize,
    idles: VecDeque<Connection>,
    closed: bool,
}

impl ConnectionPool {
    pub fn new(capacity: usize) -> ConnectionPool {
        ConnectionPool {
            addr: String::from("127.0.0.1:6379"),
            capacity,
            idles: VecDeque::with_capacity(capacity),
            closed: false,
        }
    }

    pub fn get(&mut self) -> Result<Connection> {
        if self.closed {
            return Err(Error::ConnectionPoolClosed);
        }

        if let Some(conn) = self.idles.pop_front() {
            return Ok(conn);
        }
        Connection::connect(&self.addr)
    }

    // pub fn put_conn(conn: Connection) {
    //     todo!()
    // }
}
