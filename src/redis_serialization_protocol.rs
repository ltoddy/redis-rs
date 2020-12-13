pub trait RedisSerializationProtocol {
    fn serialization(&self) -> Vec<u8>;

    fn deserialization<Output>(data: Vec<u8>) -> Output;
}

impl RedisSerializationProtocol for &str {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(b"*1\r\n");
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for String {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(b"*1\r\n");
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for u64 {
    fn serialization(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn deserialization<Output>(data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for bool {
    fn serialization(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn deserialization<Output>(data: Vec<u8>) -> Output {
        unimplemented!()
    }
}
