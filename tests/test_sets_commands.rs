use redisclient::RedisClient;

#[test]
pub fn set_sadd() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", vec!["Hello", "World", "World"]);
    let members = client.smembers("myset").unwrap();

    client.flushall().unwrap();
}
