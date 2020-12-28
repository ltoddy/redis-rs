redis-rs
========

![ci](https://github.com/ltoddy/redis-rs/workflows/ci/badge.svg)
![latest version](https://img.shields.io/crates/v/redisclient.svg)
![doc](https://docs.rs/redisclient/badge.svg)

Redis client for Rust.

- Pure Rust, and doesn't depend on any 3rd party libraries

*Cargo.toml*

```toml
[dependencies.redisclient]
version = "*"
```

*src/main.rs*

```rust
extern crate redisclient;

use redisclient::RedisResult;
use redisclient::RedisClient;

fn main() {
    if let Err(e) = run() {
        println!("Error -> {}", e);
    }
}

fn run() -> RedisResult<()> {
    let mut client = RedisClient::new()?;
    client.mset(vec![("key1", 1), ("key2", 2)])?;

    let values: Vec<String> = client.mget(vec!["key1", "key2"])?;
    println!("values -> {:?}", values);

    let values: Vec<isize> = client.mget(vec!["key1", "key2"])?;
    println!("values -> {:?}", values);

    Ok(())
}
```
