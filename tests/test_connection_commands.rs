use redisclient::client::RedisClient;

#[test]
pub fn test_echo() {
    let mut client = RedisClient::new().unwrap();

    let resp = client.echo("Hello world").unwrap();
    assert_eq!(resp, "Hello world".to_string());
}

#[test]
pub fn test_ping() {
    let mut client = RedisClient::new().unwrap();

    let _ = client.ping().unwrap();
}
