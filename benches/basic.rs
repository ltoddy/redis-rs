#![feature(test)]

extern crate test;

use test::Bencher;

use redisclient::client::RedisClient;

#[bench]
fn bench_strings_command(b: &mut Bencher) {
    let mut client = RedisClient::new().unwrap();

    b.iter(|| {
        let key = "test_key";
        client.simple_set(key, 42).unwrap();
        let _: isize = client.get(key).unwrap();
        client.del(vec![key]).unwrap();
    });
}
