mod maps;
mod numbers;
mod sequences;
mod sets;
mod strings;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;

use crate::client::ListBeforeOrAfter;
use crate::connection::{Reply, SingleStrings};
use crate::error::ErrorKind::{ResponseError, TypeError};
use crate::error::RedisError;
use crate::implement_deserialization_for_maps;
use crate::implement_deserialization_for_numbers;
use crate::implement_deserialization_for_sets;
use crate::implement_deserialization_for_string;
use crate::implement_serialization_for_maps;
use crate::implement_serialization_for_numbers;
use crate::implement_serialization_for_sets;
use crate::implement_serialization_for_string;
use crate::DataType;
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

implement_deserialization_for_string!(String);
implement_serialization_for_string!(String, &str);

implement_serialization_for_numbers!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
implement_deserialization_for_numbers!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

implement_serialization_for_maps!(HashMap, BTreeMap);
implement_deserialization_for_maps!(HashMap, BTreeMap);

implement_serialization_for_sets!(HashSet, BTreeSet);
implement_deserialization_for_sets!(HashSet); // TODO: BTreeSet, Ord trait

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

implement_serialization_for_sequences!(Vec<u8>);

impl RedisSerializationProtocol for ListBeforeOrAfter {
    fn serialization(&self) -> Vec<u8> {
        match self {
            ListBeforeOrAfter::Before => "BEFORE".serialization(),
            ListBeforeOrAfter::After => "AFTER".serialization(),
        }
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
