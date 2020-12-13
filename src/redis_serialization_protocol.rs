pub trait RedisSerializationProtocol {
    fn serialization(&self) -> Vec<u8>;

    fn deserialization<Output>(_data: Vec<u8>) -> Output;
}

// TODO: use macro
impl RedisSerializationProtocol for &str {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(_data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for String {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(_data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for i64 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(_data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for u64 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(_data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for Vec<u8> {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_slice());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(_data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for f32 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(_data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for f64 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn deserialization<Output>(_data: Vec<u8>) -> Output {
        unimplemented!()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_vector() {
        let data = b"Hello world".to_vec();

        let got = data.serialization();

        let expected = Vec::from("$11\r\nHello world\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_string() {
        let s = String::from("Hello world");

        let got = s.serialization();

        let expected = vec![
            36, 49, 49, 13, 10, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 13, 10,
        ];
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_u64() {
        let num: u64 = 132;

        let got = num.serialization();

        let expected = Vec::from("$3\r\n132\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_i64() {
        let num: i64 = -321;

        let got = num.serialization();

        let expected = Vec::from("$4\r\n-321\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_f32() {
        let fnum: f32 = -1.23;

        let got = fnum.serialization();

        let expected = Vec::from("$5\r\n-1.23\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_f64() {
        let fnum: f64 = -1.23;

        let got = fnum.serialization();

        let expected = Vec::from("$5\r\n-1.23\r\n");
        assert_eq!(expected, got);
    }
}
