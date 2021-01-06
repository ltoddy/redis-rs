pub trait ToRedisConnectionConfig {
    fn to_redis_connection_config(&self) -> RedisConfig;
}

// TODO: impl TORedisConnectionConfig for String, &str and so on.

pub struct RedisConfig {
    pub address: String,
    pub database: u8,
    pub username: Option<String>,
    pub password: Option<String>,
    pub pool_capacity: usize,
}

impl RedisConfig {
    pub fn new(
        address: String,
        database: u8,
        username: Option<String>,
        password: Option<String>,
        pool_capacity: usize,
    ) -> Self {
        RedisConfig {
            address,
            database,
            username,
            password,
            pool_capacity,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            address: "127.0.0.1:6379".to_string(),
            database: 0,
            username: None,
            password: None,
            pool_capacity: 8,
        }
    }
}
