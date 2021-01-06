use redisclient::RedisClient;

pub fn test_zadd() {
    let mut client = RedisClient::new().unwrap();

    assert_eq!(client.zadd("myzset", vec![(1, "one")]).unwrap(), 1);
    assert_eq!(client.zadd("myzset", vec![(1, "uno")]).unwrap(), 1);
    assert_eq!(client.zadd("myzset", vec![(2, "two"), (3, "three")]).unwrap(), 2);

    // let _ = client.zrange("myzset", 0, -1, "WITHSCORES");

    client.flushall().unwrap();
}

#[test]
pub fn test_zcard() {
    let mut client = RedisClient::new().unwrap();

    assert_eq!(client.zadd("myzset", vec![(1, "one"), (2, "two")]).unwrap(), 2);
    assert_eq!(client.zcard("myzset").unwrap(), 2);

    client.flushall().unwrap();
}

#[test]
pub fn test_zcount() {
    let mut client = RedisClient::new().unwrap();

    assert_eq!(
        client
            .zadd("myzset", vec![(1, "one"), (2, "two"), (3, "three")])
            .unwrap(),
        3
    );

    assert_eq!(
        client.zcount("myzset", isize::min_value(), isize::max_value()).unwrap(),
        3
    );
    assert_eq!(client.zcount("myzset", 2, 3).unwrap(), 2);

    client.flushall().unwrap();
}
