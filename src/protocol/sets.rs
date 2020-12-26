#[macro_export]
macro_rules! implement_serialization_for_sets {
    ($($t:ident),*) => {
        $(
            impl<T> RedisSerializationProtocol for $t<T>
            where
                T: RedisSerializationProtocol + Hash + Eq,
            {
                fn serialization(&self) -> Vec<u8> {
                    let length = self.len();
                    let mut buf = Vec::new();
                    buf.extend(b"1\r\n");
                    buf.extend(format!("${}\r\n", length).as_bytes());
                    for member in self {
                        buf.extend(member.serialization());
                    }
                    buf
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! implement_deserialization_for_sets {
    ($($t:ident),*) => {
        $(
            impl<T> RedisDeserializationProtocol for $t<T>
            where
                T: RedisDeserializationProtocol + Hash + Eq,
            {
                fn deserialization(reply: Reply) -> RedisResult<Self> {
                    match reply {
                        Reply::Arrays(array) => {
                            let mut set = $t::<T>::new();
                            for reply in array {
                                let element = <T>::deserialization(reply)?;
                                set.insert(element);
                            }
                            Ok(set)
                        }
                        _ => Err(RedisError::custom(TypeError, "miss type")),
                    }
                }
            }
        )*
    };
}
