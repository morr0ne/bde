use crate::{ByteString, Error, Result};
use serde::de::{Deserialize, DeserializeSeed, MapAccess};

use super::Deserializer;

pub struct MapDeserializer<'a, 'de: 'a> {
    deserializer: &'a mut Deserializer<'de>,
    last_key: Option<ByteString>,
}

impl<'a, 'de> MapDeserializer<'a, 'de> {
    pub fn new(deserializer: &'a mut Deserializer<'de>) -> Self {
        Self {
            deserializer,
            last_key: None,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for MapDeserializer<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.deserializer.peek_byte()? {
            b'e' => {
                self.deserializer.advance();
                Ok(None)
            }
            b'0'..=b'9' => {
                // seed.deserialize(&mut *self).map(Some)
                /*
                HACK: We need to deserialize the key without actually advancing the deserializer buffer.
                Cloning is a quick way to achive this but suboptimal to say the least.
                */
                let key = ByteString::deserialize(&mut (*self.deserializer).clone())?;

                if let Some(last_key) = &self.last_key {
                    if last_key > &key {
                        Err(Error::UnsortedKeys)
                    } else {
                        seed.deserialize(&mut *self.deserializer).map(Some)
                    }
                } else {
                    self.last_key = Some(key);

                    seed.deserialize(&mut *self.deserializer).map(Some)
                }
            }
            token => Err(Error::unexpected_token(
                "number between 0-9",
                token,
                self.deserializer.index,
            )),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}
