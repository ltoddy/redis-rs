use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use crate::DataType;

use crate::client::ListBeforeOrAfter;
use crate::connection::{Reply, SingleStrings};
use crate::error::ErrorKind::{ResponseError, TypeError};
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
                    buf.extend(format!("${}\r\n", length).as_bytes());
                    buf.extend(self.as_bytes());
                    buf.extend(b"\r\n");
                    buf
                }
            }
        )*
    };
}

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

macro_rules! implement_serialization_for_sequences {
    ($($t:ty),*) => {
        $(
            impl RedisSerializationProtocol for $t {
                fn serialization(&self) -> Vec<u8> {
                    let length = self.len();
                    let mut buf = Vec::new();
                    buf.extend(b"*1\r\n");
                    buf.extend(format!("${}\r\n", length).as_bytes());
                    buf.extend(self);
                    buf.extend(b"\r\n");
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

implement_serialization_for_string!(String, &str);
implement_deserialization_for_string!(String);
implement_serialization_for_numbers!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
implement_deserialization_for_numbers!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
implement_serialization_for_sequences!(Vec<u8>); // TODO VecDequeue, LinkedList
implement_deserialization_for_maps!(HashMap, BTreeMap);
// TODO:
// 1. Sequence: Vec, VecDequeue, LinkedList
// 2. Maps: HashMap, BTreeMap,
// 3. Sets: HashSet, BTreeSet

impl RedisSerializationProtocol for ListBeforeOrAfter {
    fn serialization(&self) -> Vec<u8> {
        match self {
            ListBeforeOrAfter::Before => "BEFORE".serialization(),
            ListBeforeOrAfter::After => "AFTER".serialization(),
        }
    }
}

impl<K, V> RedisSerializationProtocol for HashMap<K, V>
where
    K: RedisSerializationProtocol,
    V: RedisSerializationProtocol,
{
    fn serialization(&self) -> Vec<u8> {
        unimplemented!()
    }
}

impl RedisDeserializationProtocol for () {
    fn deserialization(reply: Reply) -> RedisResult<Self> {
        match reply {
            Reply::SingleStrings(single) => match single {
                SingleStrings::Okay | SingleStrings::Pong => Ok(()),
                _ => Err(RedisError::custom(ResponseError, "wrong data type")),
            },
            _ => Err(RedisError::custom(TypeError, "miss type")),
        }
    }
}

impl RedisDeserializationProtocol for bool {
    fn deserialization(reply: Reply) -> RedisResult<Self> {
        let v = <usize>::deserialization(reply)?;
        Ok(v != 0)
    }
}

impl<T> RedisDeserializationProtocol for Vec<T>
where
    T: RedisDeserializationProtocol,
{
    fn deserialization(reply: Reply) -> RedisResult<Self> {
        match reply {
            Reply::Arrays(array) => {
                let mut values = Vec::new();
                for ele in array {
                    values.push(<T>::deserialization(ele)?);
                }
                Ok(values)
            }
            _ => Err(RedisError::custom(TypeError, "miss type")),
        }
    }
}

impl RedisDeserializationProtocol for DataType {
    fn deserialization(reply: Reply) -> RedisResult<Self> {
        match reply {
            Reply::SingleStrings(single) => match single {
                SingleStrings::String => Ok(DataType::String),
                SingleStrings::List => Ok(DataType::List),
                SingleStrings::Set => Ok(DataType::Set),
                _ => Err(RedisError::custom(ResponseError, "wrong data type")),
            },
            _ => Err(RedisError::custom(TypeError, "miss type")),
        }
    }
}
