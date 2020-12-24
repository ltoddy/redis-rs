use redisclient::client::ListBeforeOrAfter::Before;
use redisclient::RedisClient;

#[test]
pub fn test_lindex() {
    let mut client = RedisClient::new().unwrap();

    client.lpush("mylist", vec!["World"]).unwrap();
    client.lpush("mylist", vec!["Hello"]).unwrap();

    assert_eq!(
        client.lindex::<&'static str, String>("mylist", 0).unwrap(),
        "Hello".to_string()
    );
    assert_eq!(
        client.lindex::<&'static str, String>("mylist", -1).unwrap(),
        "World".to_string()
    );
    assert_eq!(
        client.lindex::<&'static str, String>("mylist", 3).unwrap(),
        "".to_string()
    );

    client.flushall().unwrap();
}

#[test]
pub fn test_linsert() {
    let mut client = RedisClient::new().unwrap();

    client.rpush("mylist", vec!["Hello"]).unwrap();
    client.rpush("mylist", vec!["World"]).unwrap();

    let pos = client.linsert("mylist", Before, "World", "There").unwrap();
    assert_eq!(pos, 3);

    assert_eq!(
        client.lrange::<&'static str, String>("mylist", 0, -1).unwrap(),
        vec!["Hello".to_string(), "There".to_string(), "World".to_string()]
    );

    client.flushall().unwrap();
}

#[test]
pub fn test_llen() {
    let mut client = RedisClient::new().unwrap();

    client.lpush("mylist", vec!["World"]).unwrap();
    client.lpush("mylist", vec!["Hello"]).unwrap();

    assert_eq!(client.llen("mylist").unwrap(), 2);

    client.flushall().unwrap();
}

#[test]
pub fn test_lpop() {
    let mut client = RedisClient::new().unwrap();

    client.rpush("mylist", vec!["one", "two", "three"]).unwrap();
    let element: String = client.lpop("mylist").unwrap();
    assert_eq!(element, "one".to_string());

    let elements: Vec<String> = client.lrange("mylist", 0, -1).unwrap();
    assert_eq!(elements, vec!["two".to_string(), "three".to_string()]);

    client.flushall().unwrap();
}

#[test]
pub fn test_lpush() {
    let mut client = RedisClient::new().unwrap();

    client.lpush("mylist", vec!["world", "hello"]).unwrap();
    let elements: Vec<String> = client.lrange("mylist", 0, -1).unwrap();

    assert_eq!(elements, vec!["hello".to_string(), "world".to_string()]);

    client.flushall().unwrap();
}

#[test]
pub fn test_lpushx() {
    let mut client = RedisClient::new().unwrap();

    client.lpush("mylist", vec!["World"]).unwrap();

    assert_eq!(client.lpushx("mylist", vec!["Hello"]).unwrap(), 2);
    assert_eq!(client.lpushx("myotherlist", vec!["Hello"]).unwrap(), 0);

    assert_eq!(
        client.lrange::<&'static str, String>("mylist", 0, -1).unwrap(),
        vec!["Hello".to_string(), "World".to_string()]
    );

    assert_eq!(
        client.lrange::<&'static str, String>("myotherlist", 0, -1).unwrap(),
        Vec::<String>::new()
    );

    client.flushall().unwrap();
}

#[test]
pub fn test_lrange() {
    let mut client = RedisClient::new().unwrap();
    client.rpush("mylist", vec!["one", "two", "three"]).unwrap();

    let elements: Vec<String> = client.lrange("mylist", 0, 0).unwrap();
    assert_eq!(elements, vec!["one".to_string()]);
    let elements: Vec<String> = client.lrange("mylist", -3, 2).unwrap();
    assert_eq!(
        elements,
        vec!["one".to_string(), "two".to_string(), "three".to_string()]
    );
    client.flushall().unwrap();
}

#[test]
pub fn test_lrem() {
    let mut client = RedisClient::new().unwrap();
    client.rpush("mylist", vec!["hello", "hello", "foo", "hello"]).unwrap();

    let amount = client.lrem("mylist", -2, "hello").unwrap();
    assert_eq!(amount, 2);

    let elements: Vec<String> = client.lrange("mylist", 0, -1).unwrap();
    assert_eq!(elements, vec!["hello".to_string(), "foo".to_string()]);

    client.flushall().unwrap();
}

#[test]
pub fn test_lset() {
    let mut client = RedisClient::new().unwrap();
    client.rpush("mylist", vec!["onw", "two", "three"]).unwrap();
    client.lset("mylist", 0, "four").unwrap();
    client.lset("mylist", -2, "five").unwrap();

    let elements: Vec<String> = client.lrange("mylist", 0, -1).unwrap();
    assert_eq!(
        elements,
        vec!["four".to_string(), "five".to_string(), "three".to_string()]
    );

    client.flushall().unwrap();
}

#[test]
pub fn rpop() {
    let mut client = RedisClient::new().unwrap();

    client.rpush("mylist", vec!["one", "two", "three"]).unwrap();
    let element: String = client.rpop("mylist").unwrap();
    assert_eq!(element, "three".to_string());

    let elements: Vec<String> = client.lrange("mylist", 0, -1).unwrap();
    assert_eq!(elements, vec!["one".to_string(), "two".to_string()]);

    client.flushall().unwrap();
}
