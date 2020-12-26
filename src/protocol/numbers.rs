#[macro_export]
macro_rules! implement_serialization_for_numbers {
    ($($t:ty),*) => {
        $(
            impl RedisSerializationProtocol for $t {
                fn serialization(&self) -> Vec<u8> {
                    let s = format!("{}", self);
                    let length = s.len();
                    let mut buf = Vec::new();
                    buf.extend(format!("${}\r\n", length).as_bytes());
                    buf.extend(s.as_bytes());
                    buf.extend(b"\r\n");
                    buf
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! implement_deserialization_for_numbers {
    ($($t:ty),*) => {
        $(
            impl RedisDeserializationProtocol for $t {
                fn deserialization(reply: Reply) -> RedisResult<Self> {
                    match reply {
                        Reply::Integers(data) => Ok(String::from_utf8(data)?.parse::<$t>()?),
                        Reply::BulkStrings(data) => Ok(String::from_utf8(data)?.parse::<$t>()?),
                        _ => Err(RedisError::custom(TypeError, "miss type")),
                    }
                }
            }
        )*
    };
}
