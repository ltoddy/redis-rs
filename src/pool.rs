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

    pub fn put(&mut self, conn: Connection) {
        if self.closed {
            return;
        }

        if self.idles.len() >= self.capacity {
            let _ = self.idles.pop_front();
        }
        self.idles.push_back(conn);
    }

    pub fn close(&mut self) {
        self.closed = true;

        while let Some(conn) = self.idles.pop_front() {
            drop(conn);
        }
    }
}
