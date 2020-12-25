use std::collections::HashSet;

use redisclient::hash_set;
use redisclient::RedisClient;

#[test]
pub fn set_sadd() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", hash_set!["Hello", "World", "World"]).unwrap();
    let members: HashSet<String> = client.smembers("myset").unwrap();
    assert_eq!(members, hash_set!("Hello".to_string(), "World".to_string()));

    client.flushall().unwrap();
}

#[test]
pub fn test_scard() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", hash_set!["Hello", "World"]).unwrap();
    let amount = client.scard("myset").unwrap();
    assert_eq!(amount, 2);

    client.flushall().unwrap();
}

#[test]
pub fn test_sdiff() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("key1", hash_set!["a", "b", "c"]).unwrap();
    client.sadd("key2", hash_set!["c", "d", "e"]).unwrap();
    let diff: HashSet<String> = client.sdiff(vec!["key1", "key2"]).unwrap();
    assert_eq!(diff, hash_set!("a".to_string(), "b".to_string()));

    client.flushall().unwrap();
}

#[test]
pub fn test_sdiffstore() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("key1", hash_set!["a", "b", "c"]).unwrap();
    client.sadd("key2", hash_set!["c", "d", "e"]).unwrap();
    client.sdiffstore("key", vec!["key1", "key2"]).unwrap();
    let elements: HashSet<String> = client.smembers("key").unwrap();

    assert_eq!(elements, hash_set!("a".to_string(), "b".to_string()));

    client.flushall().unwrap();
}

#[test]
pub fn sinter() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("key1", hash_set!("a", "b", "c")).unwrap();
    client.sadd("key2", hash_set!("c", "d", "e")).unwrap();
    let members: HashSet<String> = client.sinter(vec!["key1", "key2"]).unwrap();

    assert_eq!(members, hash_set!("c".to_string()));

    client.flushall().unwrap();
}

#[test]
pub fn test_sinterstore() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("key1", hash_set!("a", "b", "c")).unwrap();
    client.sadd("key2", hash_set!("c", "d", "e")).unwrap();

    let amount = client.sinterstore("key", vec!["key1", "key2"]).unwrap();
    assert_eq!(amount, 1);
    let elements: HashSet<String> = client.smembers("key").unwrap();
    assert_eq!(elements, hash_set!("c".to_string()));

    client.flushall().unwrap();
}



