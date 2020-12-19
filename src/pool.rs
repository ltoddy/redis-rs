use std::collections::VecDeque;

use crate::connection::Connection;
use crate::error::{ErrorKind, RedisError};
use crate::RedisResult;

pub struct ConnectionPool {
    addr: String,
    capacity: usize,
    idles: VecDeque<Connection>,
    closed: bool,
}

impl ConnectionPool {
    pub(super) fn new(capacity: usize) -> ConnectionPool {
        ConnectionPool {
            addr: String::from("127.0.0.1:6379"),
            capacity,
            idles: VecDeque::with_capacity(capacity),
            closed: false,
        }
    }

    pub(super) fn get(&mut self) -> RedisResult<Connection> {
        if self.closed {
            return Err(RedisError::custom(ErrorKind::ClientError, "Connection pool closed"));
        }

        if let Some(conn) = self.idles.pop_front() {
            return Ok(conn);
        }
        Connection::connect(&self.addr)
    }

    pub(super) fn put(&mut self, conn: Connection) {
        if self.closed {
            return;
        }

        if self.idles.len() >= self.capacity {
            let _ = self.idles.pop_front();
        }
        self.idles.push_back(conn);
    }

    fn close(&mut self) {
        self.closed = true;

        while let Some(conn) = self.idles.pop_front() {
            drop(conn);
        }
    }
}

impl Drop for ConnectionPool {
    fn drop(&mut self) {
        self.close()
    }
}
