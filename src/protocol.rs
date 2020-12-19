use crate::connection::{Reply, SingleStrings::Okay};
use crate::error::ErrorKind::TypeError;
use crate::error::RedisError;
use crate::RedisResult;

pub trait RedisSerializationProtocol {
    fn serialization(&self) -> Vec<u8>;
}

pub trait RedisDeserializationProtocol {
    fn deserialization(reply: Reply) -> RedisResult<Self>
    where
        Self: Sized;
}

// ---------------------------------------

macro_rules! implement_serialization_for_string {
    ($($t:ty),*) => {
        $(
            impl RedisSerializationProtocol for $t {
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
            impl RedisSerializationProtocol for $t {
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

macro_rules! implement_serialization_for_array {
    ($($t:ty),*) => {
        $(
            impl RedisSerializationProtocol for $t {
                fn serialization(&self) -> Vec<u8> {
                    let length = self.len();
                    let mut buf = Vec::new();
                    buf.extend_from_slice(b"*1\r\n");
                    buf.extend_from_slice(format!("${}\r\n", length).as_bytes());
                    buf.extend_from_slice(self);
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
            impl RedisDeserializationProtocol for $t {
                fn deserialization(reply: Reply) -> RedisResult<Self> {
                    match reply {
                        Reply::SingleStrings(single) => {
                            match single { Okay => Ok(<$t>::new()) }
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

macro_rules! implement_deserialization_for_number {
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

implement_serialization_for_string!(String, &str);
implement_deserialization_for_string!(String);
implement_serialization_for_number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
implement_deserialization_for_number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
implement_serialization_for_array!(Vec<u8>); // TODO

impl RedisDeserializationProtocol for () {
    fn deserialization(reply: Reply) -> RedisResult<Self> {
        match reply {
            Reply::SingleStrings(single) => match single {
                Okay => Ok(()),
            },
            _ => Err(RedisError::custom(TypeError, "miss type")),
        }
    }
}

impl<T: RedisDeserializationProtocol> RedisDeserializationProtocol for Vec<T> {
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
