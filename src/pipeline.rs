use crate::client::Command;
use crate::RedisClient;

#[allow(dead_code)]
pub struct Pipeline<'a> {
    client: &'a RedisClient, // TODO
    commands: Vec<Command>,
    transaction: bool,
}

impl<'a> Pipeline<'a> {
    pub(crate) fn new(client: &RedisClient) -> Pipeline {
        Self::with_capacity(client, 0)
    }

    pub(crate) fn with_capacity(client: &RedisClient, capacity: usize) -> Pipeline {
        Pipeline {
            client,
            commands: Vec::with_capacity(capacity),
            transaction: false,
        }
    }

    pub fn transaction_mode(&'a mut self) -> &mut Pipeline {
        self.transaction = true;
        self
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}
