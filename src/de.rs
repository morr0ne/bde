use crate::{ByteString, Error, Result};
use num_traits::{cast::AsPrimitive, NumCast, PrimInt, WrappingNeg};
use paste::paste;
use serde::{
    de::{
        self, value::SeqDeserializer, Deserialize, DeserializeSeed, IntoDeserializer, MapAccess,
        SeqAccess, Visitor,
    },
    serde_if_integer128,
};

macro_rules! deserialize_integers {
    ($($ty:ident)+) => {
        $(
            paste! {
                fn [<deserialize_$ty>]<V>(self, visitor: V) -> Result<V::Value>
                where
                    V: de::Visitor<'de>,
                {
                    if let b'i' = self.next_byte()? {
                        match self.peek_byte()? {
                            b'-' => {
                                self.advance();
                                visitor.[<visit_$ty>](self.parse_integer(true)?)
                            }
                            _ => visitor.[<visit_$ty>](self.parse_integer(false)?),
                        }
                    } else {
                        Err(Error::InvalidType)
                    }
                }
            }
        )*
    }
}

macro_rules! deserialize_unsigned_integers {
    ($($ty:ident)+) => {
        $(
            paste! {
                fn [<deserialize_$ty>]<V>(self, visitor: V) -> Result<V::Value>
                where
                    V: Visitor<'de>,
                {
                    if let b'i' = self.next_byte()? {
                        visitor.[<visit_$ty>](self.parse_integer(false)?)
                    } else {
                        Err(Error::InvalidType)
                    }
                }
            }
        )*
    }
}

pub struct Deserializer<'de> {
    bytes: &'de [u8],
    index: usize,
}

impl<'de> Deserializer<'de> {
    /// Create a new derializer.
    pub const fn from_bytes(bytes: &'de [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    /// Returns the next byte and advances the internal buffer by one.
    /// Returns None if empty.
    fn next_byte(&mut self) -> Result<u8> {
        if let Some(byte) = self.bytes.get(self.index) {
            self.advance();
            Ok(*byte)
        } else {
            Err(Error::Eof)
        }
    }

    /// Look at the next byte without advancing the buffer.
    /// Returns None if empty.
    fn peek_byte(&mut self) -> Result<u8> {
        if let Some(byte) = self.bytes.get(self.index) {
            Ok(*byte)
        } else {
            Err(Error::Eof)
        }
    }

    /// Advances the internal buffer by one
    fn advance(&mut self) {
        self.index += 1;
    }

    /// Ensures there aren't any trailing bytes
    /// # Errors
    /// TODO
    pub fn finish(&mut self) -> Result<()> {
        if self.bytes.len() > self.index {
            Err(Error::TrailingBytes)
        } else {
            Ok(())
        }
    }

    /// Parses ascii numbers into as type N until a certain byte is found and discards it
    fn next_ascii_number_until<N>(&mut self, negative: bool, until: u8) -> Result<N>
    where
        N: Copy + PrimInt + NumCast + WrappingNeg + 'static,
        u8: AsPrimitive<N>,
        i8: num_traits::AsPrimitive<N>,
    {
        let mut significand = N::zero();

        loop {
            match self.next_byte()? {
                integer @ b'0'..=b'9' => {
                    // To convert an ascii number to an actuall number we can just subtract the ascii rappresentation of 0
                    let digit = (integer - b'0').as_();

                    let max = N::max_value();

                    // Checks if the number would overflow
                    if significand >= max / 10u8.as_()
                        && (significand > max / 10u8.as_() || digit > max % 10u8.as_())
                    {
                        return Err(Error::OutOfBound);
                    } else {
                        significand = significand * 10u8.as_() + digit;
                    }
                }
                token => {
                    break if token != until {
                        Err(Error::unexpected_token("", token, self.index))
                    } else if negative {
                        Ok(significand.wrapping_neg())
                    } else {
                        Ok(significand)
                    }
                }
            }
        }
    }

    /// Parses any integer ignoring the leading "i" bytes
    fn parse_integer<N>(&mut self, negative: bool) -> Result<N>
    where
        N: Copy + PrimInt + NumCast + WrappingNeg + 'static,
        u8: AsPrimitive<N>,
        i8: num_traits::AsPrimitive<N>,
    {
        // Check the first byte, if none then we hit eof too soon
        match self.peek_byte()? {
            b'0' => {
                if negative {
                    Err(Error::NegativeZero) // Negative zero is an invalid bencode number
                } else {
                    self.advance();
                    match self.next_byte()? {
                        b'e' => Ok(N::zero()),
                        b'0'..=b'9' => Err(Error::LeadingZero), // The only valid case for a leading zero is simply 0, any other number is invalid
                        token => Err(Error::unexpected_token("e", token, self.index)), // The only possible valid token at the end is "e"
                    }
                }
            }
            b'1'..=b'9' => self.next_ascii_number_until(negative, b'e'),
            token => Err(Error::unexpected_token(
                "number between 0-9",
                token,
                self.index,
            )),
        }
    }

    /// Parses a byte string
    fn parse_byte_string(&mut self) -> Result<&'de [u8]> {
        let len = self.next_ascii_number_until::<usize>(false, b':')?;

        if len == 0 {
            return Ok(&[]);
        }

        if let Some(computed_index) = self.index.checked_add(len) {
            if self.bytes.len() >= (computed_index) {
                let bytes = &self.bytes[self.index..computed_index];
                self.index = computed_index;
                Ok(bytes)
            } else {
                Err(Error::EofWhileParsingByteString)
            }
        } else {
            Err(Error::OutOfBound)
        }
    }
}

/// # Errors
/// TODO
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(bytes);
    let value = T::deserialize(&mut deserializer)?;
    deserializer.finish()?;
    Ok(value)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek_byte()? {
            b'i' => self.deserialize_i64(visitor),
            b'0'..=b'9' => self.deserialize_bytes(visitor),
            b'l' => self.deserialize_seq(visitor),
            b'd' => self.deserialize_map(visitor),
            token => Err(Error::unexpected_token(
                "one of: i, 0-9, l, d",
                token,
                self.index,
            )),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("bool"))
    }

    deserialize_integers!(i8 i16 i32 i64);
    deserialize_unsigned_integers!(u8 u16 u32 u64);
    serde_if_integer128! {
        deserialize_unsigned_integers!(u128);
        deserialize_integers!(i128);
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("f32"))
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("f64"))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!() // TODO: Parse string and check if it's of len 1
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(std::str::from_utf8(self.parse_byte_string()?)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.parse_byte_string()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_some(self) // TODO: Bencode doesn't really have a concept of missing value.
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!() // TODO: Figure out what to do here.
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!() // TODO: Figure out what to do here.
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.next_byte()? {
            b'l' => visitor.visit_seq(self),
            token => Err(Error::unexpected_token("l", token, self.index)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.next_byte()? {
            // b'd' => visitor.visit_map(self),
            b'd' => visitor.visit_map(MapDeserializer::new(self)),
            b'l' => Err(Error::ExpectedDictionaryFoundList),
            b'i' => Err(Error::ExpectedDictionaryFoundInteger),
            b'0'..=b'9' => Err(Error::ExpectedDictionaryFoundByteString),
            token => Err(Error::unexpected_token("d", token, self.index)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!() // TODO: Figure out what to do here.
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de> SeqAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if let b'e' = self.peek_byte()? {
            self.advance();
            Ok(None)
        } else {
            seed.deserialize(&mut *self).map(Some)
        }
    }
}

struct MapDeserializer<'a, 'de: 'a> {
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

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
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
                let key = ByteString::deserialize(&mut *self.deserializer)?;

                if let Some(last_key) = &self.last_key {
                    if last_key > &key {
                        Err(Error::UnsortedKeys)
                    } else {
                        let deserializer: SeqDeserializer<std::vec::IntoIter<u8>, Error> =
                            key.into_vec().into_deserializer();
                        seed.deserialize(deserializer).map(Some)
                    }
                } else {
                    self.last_key = Some(key.clone());
                    let deserializer: SeqDeserializer<std::vec::IntoIter<u8>, Error> =
                        key.into_vec().into_deserializer();
                    seed.deserialize(deserializer).map(Some)
                }
            }
            token => Err(Error::unexpected_token(
                "number between 0-9",
                token,
                self.deserializer.index,
            )),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

#[cfg(test)]
mod tests {

    use crate::from_bytes;

    #[test]
    fn zero_lenght_byte_string() {
        assert_eq!("", from_bytes::<&'static str>(b"0:").unwrap())
    }
}
