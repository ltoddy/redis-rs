pub struct RedisConfig {
    pub address: String,
    pub database: u8,
    pub password: String,
    pub pool_capacity: usize,
}

impl RedisConfig {
    pub fn new(address: String, database: u8, password: String, pool_capacity: usize) -> Self {
        RedisConfig {
            address,
            database,
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
            password: String::new(),
            pool_capacity: 8,
        }
    }
}
