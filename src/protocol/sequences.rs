// TODO
#[macro_export]
macro_rules! implement_serialization_for_sequences {
    ($($t: ident),*) => {
        $(
            impl<T> RedisSerializationProtocol for $t::<T>
            where
                T: RedisSerializationProtocol,
            {
                fn serialization(&self) -> Vec<u8> {
                    let length = self.len();
                    let mut buf = Vec::new();
                    buf.extend(b"1\r\n");
                    buf.extend(format!("${}\r\n", length).as_bytes());
                    for ele in self {
                        buf.extend(ele.serialization());
                    }
                    buf
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! implement_deserialization_for_sequences {
    ($($t: ident),*) => {};
}
