use crate::connection::{Reply, SingleStrings::Okay};
use crate::error::{ErrorKind, RedisError};
use crate::RedisResult;

pub trait Serialization {
    fn serialization(&self) -> Vec<u8>;
}

pub trait Deserialization {
    fn deserialization(reply: Reply) -> RedisResult<Self>
    where
        Self: Sized;
}

pub trait RedisProtocol: Serialization + Deserialization {}

// ---------------------------------------

macro_rules! implement_serialization_for_string {
    ($($t:ty),*) => {
        $(
            impl Serialization for $t {
                fn serialization(&self) -> Vec<u8> {
                    let length = self.len();
                    let mut buf = Vec::new();
                    buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
                    buf.extend_from_slice(self.as_bytes());
                    buf.extend_from_slice(b"\r\n");
                    buf
                }
            }
        )*
    };
}

macro_rules! implement_serialization_for_number {
    ($($t:ty),*) => {
        $(
            impl Serialization for $t {
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
        )*
    };
}

macro_rules! implement_deserialization_for_string {
    ($($t:ty),*) => {
        $(
            impl Deserialization for $t {
                fn deserialization(reply: Reply) -> RedisResult<Self> {
                    match reply {
                        Reply::SingleStrings(single) => {
                            match single { Okay => Ok(<$t>::new()) }
                        },
                        Reply::Errors(data) => Err(RedisError::custom(ErrorKind::FromServer, String::from_utf8(data)?)),
                        Reply::BulkStrings(data) => Ok(<$t>::from_utf8(data)?),
                        Reply::Nil => Ok(<$t>::new()),
                        _ => unreachable!(),
                    }
                }
            }
        )*
    };
}

macro_rules! implement_deserialization_for_number {
    ($($t:ty),*) => {
        $(
            impl Deserialization for $t {
                fn deserialization(reply: Reply) -> RedisResult<Self> {
                    match reply {
                        Reply::Errors(data) => Err(RedisError::custom(ErrorKind::FromServer, String::from_utf8(data)?)),
                        Reply::Integers(data) => Ok(String::from_utf8(data)?.parse::<$t>()?),
                        Reply::BulkStrings(data) => Ok(String::from_utf8(data)?.parse::<$t>()?),
                        _ => unreachable!(),
                    }
                }
            }
        )*
    };
}

macro_rules! implement_redis_protocol_for {
    ($($t:ty),*) => {
        $(impl RedisProtocol for $t {})*
    };
}

implement_serialization_for_string!(String, &str);
implement_deserialization_for_string!(String);
implement_serialization_for_number!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
implement_deserialization_for_number!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
implement_redis_protocol_for!(String, u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);

impl<T: Deserialization> Deserialization for Vec<T> {
    fn deserialization(reply: Reply) -> RedisResult<Self> {
        match reply {
            Reply::Arrays(array) => {
                let mut values = Vec::new();
                for ele in array {
                    values.push(<T>::deserialization(ele)?);
                }
                Ok(values)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // #[test]
    // pub fn test_vector_serialization() {
    //     let data = b"Hello world".to_vec();
    //
    //     let got = data.serialization();
    //
    //     let expected = Vec::from("$11\r\nHello world\r\n");
    //     assert_eq!(expected, got);
    // }

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
        let reply = Reply::Integers(vec![54, 48]);

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
