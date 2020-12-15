use crate::connection::Reply;
use crate::connection::ReplyKind;
use crate::error::Error;
use crate::Result;

pub trait Serialization {
    fn serialization(&self) -> Vec<u8>;
}

pub trait Deserialization {
    fn deserialization(reply: Reply) -> Result<Self>
    where
        Self: Sized;
}

pub trait RedisSerializationProtocol: Serialization + Deserialization {}

impl Serialization for String {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl Deserialization for String {
    fn deserialization(reply: Reply) -> Result<Self> {
        let Reply { kind, data } = reply;

        match kind {
            ReplyKind::SingleStrings => return Ok(String::from_utf8(data)?),
            ReplyKind::BulkStrings => return Ok(String::from_utf8(data)?),

            _ => "",
        };

        Ok(String::new())
    }
}

impl RedisSerializationProtocol for String {}

impl Serialization for &str {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl Deserialization for u8 {
    fn deserialization(reply: Reply) -> Result<Self> {
        let Reply { kind, data } = reply;
        match kind {
            ReplyKind::Integers => Ok(String::from_utf8(data)?.parse::<u8>()?),
            _ => Err(Error::ParseRedisReply(format!(""))),
        }
    }
}

impl Serialization for i64 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl Deserialization for i64 {
    fn deserialization(reply: Reply) -> Result<Self> {
        let Reply { kind, data } = reply;
        match kind {
            ReplyKind::Integers | ReplyKind::BulkStrings => Ok(String::from_utf8(data)?.parse::<i64>()?),
            _ => Err(Error::ParseRedisReply(format!(""))),
        }
    }
}

impl RedisSerializationProtocol for i64 {}

impl Serialization for u64 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl Deserialization for u64 {
    fn deserialization(reply: Reply) -> Result<Self> {
        let Reply { kind, data } = reply;
        match kind {
            ReplyKind::Integers | ReplyKind::BulkStrings => Ok(String::from_utf8(data)?.parse::<u64>()?),
            _ => Err(Error::ParseRedisReply(format!(""))),
        }
    }
}

impl RedisSerializationProtocol for u64 {}

impl Serialization for Vec<u8> {
    fn serialization(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(self.as_slice());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl Deserialization for Vec<u8> {
    fn deserialization(_reply: Reply) -> Result<Self> {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for Vec<u8> {}

impl Serialization for f32 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl Deserialization for f32 {
    fn deserialization(_reply: Reply) -> Result<Self> {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for f32 {}

impl Serialization for f64 {
    fn serialization(&self) -> Vec<u8> {
        let s = format!("{}", self);
        let length = s.len();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
        buf.extend_from_slice(s.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl Deserialization for f64 {
    fn deserialization(_reply: Reply) -> Result<Self> {
        unimplemented!()
    }
}

impl RedisSerializationProtocol for f64 {}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_vector_serialization() {
        let data = b"Hello world".to_vec();

        let got = data.serialization();

        let expected = Vec::from("$11\r\nHello world\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_string_serialization() {
        let s = String::from("Hello world");

        let got = s.serialization();

        let expected = vec![
            36, 49, 49, 13, 10, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 13, 10,
        ];
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_u64_serialization() {
        let num: u64 = 132;

        let got = num.serialization();

        let expected = Vec::from("$3\r\n132\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_u64_deserialization() {
        let reply = Reply::new(ReplyKind::Integers, vec![54, 48]);

        let got = <u64>::deserialization(reply).unwrap();

        let expected = 60_u64;
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_i64_serialization() {
        let num: i64 = -321;

        let got = num.serialization();

        let expected = Vec::from("$4\r\n-321\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_f32_serialization() {
        let fnum: f32 = -1.23;

        let got = fnum.serialization();

        let expected = Vec::from("$5\r\n-1.23\r\n");
        assert_eq!(expected, got);
    }

    #[test]
    pub fn test_f64_serialization() {
        let fnum: f64 = -1.23;

        let got = fnum.serialization();

        let expected = Vec::from("$5\r\n-1.23\r\n");
        assert_eq!(expected, got);
    }
}
