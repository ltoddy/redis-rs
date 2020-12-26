#[macro_export]
macro_rules! implement_serialization_for_string {
    ($($t:ty),*) => {
        $(
            impl RedisSerializationProtocol for $t {
                fn serialization(&self) -> Vec<u8> {
                    let length = self.len();
                    let mut buf = Vec::new();
                    buf.extend(format!("${}\r\n", length).as_bytes());
                    buf.extend(self.as_bytes());
                    buf.extend(b"\r\n");
                    buf
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! implement_deserialization_for_string {
    ($($t:ty),*) => {
        $(
            impl RedisDeserializationProtocol for $t {
                fn deserialization(reply: Reply) -> RedisResult<Self> {
                    match reply {
                        Reply::SingleStrings(single) => {
                            match single { SingleStrings::Okay | SingleStrings::Pong => Ok(<$t>::new()), _ => Err(RedisError::custom(ResponseError, "wrong data type")) }
                        },
                        Reply::BulkStrings(data) => Ok(<$t>::from_utf8(data)?),
                        Reply::Nil => Ok(<$t>::new()),
                        _ => Err(RedisError::custom(TypeError, "miss type")),
                    }
                }
            }
        )*
    };
}
