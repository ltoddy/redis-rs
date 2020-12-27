use std::collections::HashSet;

use redisclient::hash_set;
use redisclient::RedisClient;

#[test]
pub fn test_sadd() {
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
pub fn test_sinter() {
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

#[test]
pub fn test_sismember() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", hash_set!["one"]).unwrap();
    assert!(client.sismember("myset", "one".to_string()).unwrap());
    assert!(!client.sismember("myset", "two".to_string()).unwrap());

    client.flushall().unwrap();
}

#[test]
#[ignore]
pub fn test_smismember() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", hash_set!("one")).unwrap();
    assert_eq!(
        client
            .smismember("myset", hash_set!("one".to_string(), "notamember".to_string()))
            .unwrap(),
        vec![true, false]
    );

    client.flushall().unwrap();
}

#[test]
pub fn test_smove() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", hash_set!("one", "two")).unwrap();
    client.sadd("myotherset", hash_set!("three")).unwrap();

    client.smove("myset", "myotherset", "two").unwrap();

    let members: HashSet<String> = client.smembers("myset").unwrap();
    assert_eq!(members, hash_set!("one".to_string()));

    let members: HashSet<String> = client.smembers("myotherset").unwrap();
    assert_eq!(members, hash_set!("two".to_string(), "three".to_string()));

    client.flushall().unwrap();
}

#[test]
pub fn test_spop() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", hash_set!("one", "two", "three")).unwrap();
    let elements: HashSet<String> = client.spop("myset", Some(3)).unwrap();
    assert_eq!(
        elements,
        hash_set!("one".to_string(), "two".to_string(), "three".to_string())
    );

    client.flushall().unwrap();
}

#[test]
#[ignore]
pub fn test_srandmember() {
    let mut client = RedisClient::new().unwrap();
    client.flushall().unwrap();
}

#[test]
pub fn test_srem() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("myset", hash_set!("one", "two", "three")).unwrap();
    assert_eq!(client.srem("myset", hash_set!("one")).unwrap(), 1);
    assert_eq!(client.srem("myset", hash_set!("four")).unwrap(), 0);

    let rest_members: HashSet<String> = client.smembers("myset").unwrap();
    assert_eq!(rest_members, hash_set!("two".to_string(), "three".to_string()));

    client.flushall().unwrap();
}

#[test]
pub fn test_sunion() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("key1", hash_set!("a", "b")).unwrap();
    client.sadd("key2", hash_set!("c", "d", "e")).unwrap();

    let members: HashSet<String> = client.sunion(vec!["key1", "key2"]).unwrap();
    assert_eq!(
        members,
        hash_set!(
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string()
        )
    );

    client.flushall().unwrap();
}

#[test]
pub fn test_sunionstore() {
    let mut client = RedisClient::new().unwrap();

    client.sadd("key1", hash_set!("a", "b", "c")).unwrap();
    client.sadd("key2", hash_set!("c", "d", "e")).unwrap();

    let amount = client.sunionstore("key", vec!["key1", "key2"]).unwrap();
    assert_eq!(amount, 5);
    let members: HashSet<String> = client.smembers("key").unwrap();
    assert_eq!(
        members,
        hash_set!(
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string()
        )
    );

    client.flushall().unwrap();
}
