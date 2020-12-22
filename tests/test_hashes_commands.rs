use redisclient::RedisClient;
use redisclient::{btree_map, hash_map};

use std::collections::{BTreeMap, HashMap};

#[test]
pub fn test_hel() {
    let mut client = RedisClient::new().unwrap();

    client.hset("myhash", "field1", "foo").unwrap();

    let res = client.hdel("myhash", vec!["field1"]).unwrap();
    assert_eq!(res, 1);

    let res = client.hdel("myhash", vec!["field1"]).unwrap();
    assert_eq!(res, 0);

    client.flushall().unwrap();
}

#[test]
pub fn test_hexists() {
    let mut client = RedisClient::new().unwrap();

    client.hset("myhash", "field1", "foo").unwrap();
    let exist = client.hexists("myhash", "field1").unwrap();
    assert_eq!(exist, true);

    let exist = client.hexists("myhash", "field0").unwrap();
    assert_eq!(exist, false);

    client.flushall().unwrap();
}

#[test]
pub fn test_hget() {
    let mut client = RedisClient::new().unwrap();

    client.hset("myhash", "field1", "foo").unwrap();

    let value: String = client.hget("myhash", "field1").unwrap();
    assert_eq!(value, String::from("foo"));

    let value: String = client.hget("myhash", "field2").unwrap();
    assert_eq!(value, String::new());

    client.flushall().unwrap();
}

#[test]
pub fn test_hgetall() {
    let mut client = RedisClient::new().unwrap();
    client.hset("myhash", "field1", "Hello").unwrap();
    client.hset("myhash", "field2", "World").unwrap();

    let hash: HashMap<String, String> = client.hgetall("myhash").unwrap();

    assert_eq!(
        hash,
        hash_map! {
            "field1".to_string() => "Hello".to_string(),
            "field2".to_string() => "World".to_string(),
        }
    );

    let map: BTreeMap<String, String> = client.hgetall("myhash").unwrap();
    assert_eq!(
        map,
        btree_map! {
            "field1".to_string() => "Hello".to_string(),
            "field2".to_string() => "World".to_string(),
        }
    );

    client.flushall().unwrap();
}

#[test]
pub fn test_hincrby() {
    let mut client = RedisClient::new().unwrap();
    client.hset("myhash", "field", 5).unwrap();

    let value = client.hincrby("myhash", "field", 1).unwrap();
    assert_eq!(value, 6);

    let value = client.hincrby("myhash", "field", -1).unwrap();
    assert_eq!(value, 5);

    let value = client.hincrby("myhash", "field", -10).unwrap();
    assert_eq!(value, -5);

    client.flushall().unwrap();
}

#[test]
pub fn test_hincrbyfloat() {
    let mut client = RedisClient::new().unwrap();
    client.hset("mykey", "field", 10.50).unwrap();

    let value = client.hincrbyfloat("mykey", "field", 0.1).unwrap();
    assert!((value - 10.6).abs() < f64::EPSILON);

    let value = client.hincrbyfloat("mykey", "field", -5_f64).unwrap();
    assert!((value - 5.6).abs() < f64::EPSILON);

    client.hset("mykey", "field", 5.0e3).unwrap();
    let value = client.hincrbyfloat("mykey", "field", 2.0e2).unwrap();
    assert!((value - 5200_f64).abs() < f64::EPSILON);

    client.flushall().unwrap();
}

#[test]
pub fn test_hkeys() {
    let mut client = RedisClient::new().unwrap();

    client
        .hmset("myhash", vec![("field1", "Hello"), ("field2", "World")])
        .unwrap();

    let keys: Vec<String> = client.hkeys("myhash").unwrap();
    assert_eq!(keys, vec!["field1".to_string(), "field2".to_string()]);

    client.flushall().unwrap();
}

#[test]
pub fn test_hlen() {
    let mut client = RedisClient::new().unwrap();

    client
        .hmset("myhash", vec![("field1", "Hello"), ("field2", "World")])
        .unwrap();

    let len = client.hlen("myhash").unwrap();

    assert_eq!(len, 2);

    client.flushall().unwrap();
}

#[test]
pub fn test_hmget() {
    let mut client = RedisClient::new().unwrap();

    client
        .hmset("myhash", vec![("field1", "Hello"), ("field2", "World")])
        .unwrap();

    let values: Vec<String> = client.hmget("myhash", vec!["field1", "field2", "nofield"]).unwrap();

    assert_eq!(values, vec!["Hello".to_string(), "World".to_string(), String::new()]);

    client.flushall().unwrap();
}

#[test]
pub fn test_hmset() {
    let mut client = RedisClient::new().unwrap();

    client
        .hmset("myhash", vec![("field1", "Hello"), ("field2", "World")])
        .unwrap();

    let value: String = client.hget("myhash", "field1").unwrap();
    assert_eq!(value, "Hello".to_string());

    let value: String = client.hget("myhash", "field2").unwrap();
    assert_eq!(value, "World".to_string());

    client.flushall().unwrap();
}

#[test]
pub fn test_hscan() {}

#[test]
pub fn test_hset() {
    let mut client = RedisClient::new().unwrap();

    let amount = client.hset("myhash", "field1", "Hello").unwrap();
    assert_eq!(amount, 1);

    let value: String = client.hget("myhash", "field1").unwrap();
    assert_eq!(value, String::from("Hello"));

    client.flushall().unwrap();
}

#[test]
pub fn test_hsetnx() {
    let mut client = RedisClient::new().unwrap();

    let amount = client.hsetnx("myhash", "field", "Hello").unwrap();
    assert_eq!(amount, 1);

    let amount = client.hsetnx("myhash", "field", "World").unwrap();
    assert_eq!(amount, 0);

    let value: String = client.hget("myhash", "field").unwrap();
    assert_eq!(value, String::from("Hello"));

    client.flushall().unwrap();
}

#[test]
pub fn test_hstrlen() {
    let mut client = RedisClient::new().unwrap();

    client
        .hmset("myhash", vec![("f1", "HelloWorld"), ("f2", "99"), ("f3", "-256")])
        .unwrap();

    let len = client.hstrlen("myhash", "f1").unwrap();
    assert_eq!(len, 10);

    let len = client.hstrlen("myhash", "f2").unwrap();
    assert_eq!(len, 2);

    let len = client.hstrlen("myhash", "f3").unwrap();
    assert_eq!(len, 4);

    client.flushall().unwrap();
}

#[test]
pub fn test_hvals() {
    let mut client = RedisClient::new().unwrap();

    client.hset("myhash", "field1", "Hello").unwrap();
    client.hset("myhash", "field2", "World").unwrap();

    let values: Vec<String> = client.hvals("myhash").unwrap();
    assert_eq!(values, vec!["Hello".to_string(), "World".to_string()]);

    client.flushall().unwrap();
}
