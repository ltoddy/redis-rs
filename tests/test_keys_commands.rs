use redisclient::RedisClient;

#[test]
pub fn test_del() {
    let mut client = RedisClient::new().unwrap();
    client.simple_set("key1", "Hello").unwrap();
    client.simple_set("key2", "World").unwrap();

    let amount = client.del(vec!["key1", "key2"]).unwrap();

    assert_eq!(amount, 2);

    client.flushall().unwrap();
}

// #[test]
// pub fn test_dump() {
//     let mut client = RedisClient::new().unwrap();
//
//     client.simple_set("mykey", 10).unwrap();
//     let value = client.dump("mykey").unwrap();
//
//     assert_eq!(value, r#"\u0000\xC0\n\t\u0000\xBEm\u0006\x89Z(\u0000\n"#);
//
//     client.flushall().unwrap();
// }

#[test]
pub fn test_exists() {
    let mut client = RedisClient::new().unwrap();
    client.simple_set("key1", "Hello").unwrap();

    let exist = client.exists(vec!["key1"]).unwrap();
    assert_eq!(exist, 1);

    let exist = client.exists(vec!["nosuckkey"]).unwrap();
    assert_eq!(exist, 0);

    client.simple_set("key2", "World").unwrap();

    let exist = client.exists(vec!["key1", "key2", "nosuckkey"]).unwrap();
    assert_eq!(exist, 2);

    client.flushall().unwrap();
}

#[test]
pub fn test_expire() {
    let mut client = RedisClient::new().unwrap();

    client.simple_set("mykey", "Hello").unwrap();
    assert!(client.expire("mykey", 10).unwrap());

    let rest = client.ttl("mykey").unwrap();
    assert_eq!(rest, 10);

    client.simple_set("mykey", "Hello world").unwrap();
    let rest = client.ttl("mykey").unwrap();
    assert_eq!(rest, -1);

    client.flushall().unwrap();
}

// #[test]
// pub fn test_expireat() {}

#[test]
pub fn test_keys() {
    let mut client = RedisClient::new().unwrap();

    client
        .mset(vec![("firstname", "Jack"), ("lastname", "Stuntman"), ("age", "35")])
        .unwrap();

    let keys = client.keys("*name*").unwrap();
    assert_eq!(keys, vec!["lastname".to_string(), "firstname".to_string()]);

    let keys = client.keys("a??").unwrap();
    assert_eq!(keys, vec!["age".to_string()]);

    let keys = client.keys("*").unwrap();
    assert_eq!(
        keys,
        vec!["age".to_string(), "lastname".to_string(), "firstname".to_string()]
    );

    client.flushall().unwrap();
}

#[test]
pub fn test_persist() {
    let mut client = RedisClient::new().unwrap();

    client.simple_set("mykey", "Hello").unwrap();
    client.expire("mykey", 10).unwrap();

    let rest = client.ttl("mykey").unwrap();
    assert_eq!(rest, 10);

    client.persist("mykey").unwrap();
    let rest = client.ttl("mykey").unwrap();
    assert_eq!(rest, -1);

    client.flushall().unwrap();
}

#[test]
pub fn test_pexpire() {
    let mut client = RedisClient::new().unwrap();

    client.simple_set("mykey", "Hello").unwrap();

    assert!(client.pexpire("mykey", 1500).unwrap());

    let rest = client.ttl("mykey").unwrap();
    assert_eq!(rest, 1);

    // let rest = client.pttl("mykey").unwrap();
    // assert_eq!(rest, 1499);

    client.flushall().unwrap();
}
