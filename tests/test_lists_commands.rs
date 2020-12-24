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
