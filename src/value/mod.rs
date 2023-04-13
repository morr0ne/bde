use serde::{de::DeserializeOwned, Serialize};
use std::{collections::BTreeMap, fmt::Debug};

use crate::{byte_string::ByteString, Error};

mod de;
mod integer;
mod ser;

pub use integer::Integer;
pub use ser::ValueSerializer;

pub type Dictionary<V = Value> = BTreeMap<ByteString, V>;

/// Represents any valid Bencode value.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    /// Represents a bencode byte string.
    ByteString(ByteString),
    /// Represents a bencode integer.
    Integer(Integer),
    /// Represents a bencode list.
    List(Vec<Value>),
    /// Represents a bencode dictionary.
    Dictionary(Dictionary),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ByteString(value) => f
                .debug_tuple("ByteString")
                .field(&String::from_utf8_lossy(value))
                .finish(),
            Self::Integer(value) => Debug::fmt(value, f),
            Self::List(value) => {
                f.write_str("List(")?;
                Debug::fmt(value, f)?;
                f.write_str(")")
            }
            Self::Dictionary(value) => {
                f.write_str("Dictionary(")?;
                Debug::fmt(value, f)?;
                f.write_str(")")
            }
        }
    }
}

impl Value {
    pub const fn is_byte_string(&self) -> bool {
        matches!(self, Self::ByteString(_))
    }

    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub const fn is_list(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub const fn is_dictionary(&self) -> bool {
        matches!(self, Self::Dictionary(_))
    }

    pub const fn as_byte_string(&self) -> Option<&ByteString> {
        match self {
            Self::ByteString(byte_string) => Some(byte_string),
            _ => None,
        }
    }

    pub const fn as_integer(&self) -> Option<&Integer> {
        match self {
            Self::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    pub const fn as_list(&self) -> Option<&Vec<Value>> {
        match self {
            Self::List(list) => Some(list),
            _ => None,
        }
    }

    pub const fn as_dictionary(&self) -> Option<&Dictionary> {
        match self {
            Self::Dictionary(dictionary) => Some(dictionary),
            _ => None,
        }
    }
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(ValueSerializer)
}

pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    T::deserialize(value)
}

#[cfg(test)]
mod tests {
    use super::{ByteString, Integer, Value};
    use serde_test::{assert_tokens, Token};
    use std::collections::BTreeMap;

    #[test]
    fn deserialize_and_serialize_dictionary() {
        let mut map = BTreeMap::new();
        map.insert(ByteString::from("a"), Value::Integer(Integer::from(10u64)));
        map.insert(ByteString::from("c"), Value::Integer(Integer::from(10u64)));
        map.insert(ByteString::from("d"), Value::Integer(Integer::from(10u64)));
        map.insert(ByteString::from("b"), Value::Integer(Integer::from(10u64)));
        map.insert(ByteString::from("e"), Value::Integer(Integer::from(10u64)));

        let len = map.len();

        assert_tokens(
            &Value::Dictionary(map),
            &[
                Token::Map { len: Some(len) },
                Token::Bytes(b"a"),
                Token::U64(10),
                Token::Bytes(b"b"),
                Token::U64(10),
                Token::Bytes(b"c"),
                Token::U64(10),
                Token::Bytes(b"d"),
                Token::U64(10),
                Token::Bytes(b"e"),
                Token::U64(10),
                Token::MapEnd,
            ],
        )
    }
}
