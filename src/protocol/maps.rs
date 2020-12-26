#[macro_export]
macro_rules! implement_serialization_for_maps {
    ($($t:ident),*) => {
        $(
            impl<K, V> RedisSerializationProtocol for $t<K, V>
            where
                K: RedisSerializationProtocol + Hash + Eq,
                V: RedisSerializationProtocol,
            {
                fn serialization(&self) -> Vec<u8> {
                    unimplemented!()
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! implement_deserialization_for_maps {
    ($($t:ident),*) => {
        $(
            impl<K, V> RedisDeserializationProtocol for $t::<K, V>
            where
                K: RedisDeserializationProtocol + Eq + Hash + Ord,
                V: RedisDeserializationProtocol,
            {
                fn deserialization(reply: Reply) -> RedisResult<Self> {
                    // TODO: ugly code, refactor !!!
                    match reply {
                        Reply::Arrays(array) => {
                            let hash = array
                                .chunks(2)
                                .map(|chunk| {
                                    let field = &chunk[0];
                                    let value = &chunk[1];
                                    (
                                        <K>::deserialization(field.clone()).unwrap(),
                                        <V>::deserialization(value.clone()).unwrap(), // TODO: remove clone and unwrap
                                    )
                                })
                                .collect();
                            Ok(hash)
                        }
                        _ => Err(RedisError::custom(TypeError, "miss type")),
                    }
                }
            }
        )*
    };
}
