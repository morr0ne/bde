use num_traits::{cast::AsPrimitive, NumCast, PrimInt, WrappingNeg};
use serde::{
    de::{self, DeserializeSeed, SeqAccess, Visitor},
    serde_if_integer128,
};

use super::map_deserializer::MapDeserializer;
use crate::{Error, Result};

#[derive(Clone)]
pub struct Deserializer<'de> {
    pub(super) bytes: &'de [u8],
    pub(super) index: usize,
}

impl<'de> Deserializer<'de> {
    /// Create a new derializer.
    pub const fn from_bytes(bytes: &'de [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    /// Returns the next byte and advances the internal buffer by one.
    /// Returns None if empty.
    pub(super) fn next_byte(&mut self) -> Result<u8> {
        if let Some(byte) = self.bytes.get(self.index) {
            self.advance();
            Ok(*byte)
        } else {
            Err(Error::Eof)
        }
    }

    /// Look at the next byte without advancing the buffer.
    /// Returns None if empty.
    pub(super) fn peek_byte(&mut self) -> Result<u8> {
        if let Some(byte) = self.bytes.get(self.index) {
            Ok(*byte)
        } else {
            Err(Error::Eof)
        }
    }

    /// Advances the internal buffer by one
    pub(super) fn advance(&mut self) {
        self.index += 1;
    }

    /// Ensures there aren't any trailing bytes
    /// # Errors
    /// TODO
    pub fn check_trailing_bytes(&mut self) -> Result<()> {
        if self.bytes.len() > self.index {
            Err(Error::TrailingBytes)
        } else {
            Ok(())
        }
    }

    /// Parses ascii numbers into as type N until a certain byte is found and discards it
    pub(super) fn next_ascii_number_until<N>(&mut self, negative: bool, until: u8) -> Result<N>
    where
        N: Copy + PrimInt + NumCast + WrappingNeg + 'static,
        u8: AsPrimitive<N>,
        i8: AsPrimitive<N>,
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
    pub(super) fn parse_integer<N>(&mut self, negative: bool) -> Result<N>
    where
        N: Copy + PrimInt + NumCast + WrappingNeg + 'static,
        u8: AsPrimitive<N>,
        i8: AsPrimitive<N>,
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
    pub(super) fn parse_byte_string(&mut self) -> Result<&'de [u8]> {
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

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            match self.peek_byte()? {
                b'-' => {
                    self.advance();
                    visitor.visit_i8(self.parse_integer(true)?)
                }
                _ => visitor.visit_i8(self.parse_integer(false)?),
            }
        } else {
            Err(Error::InvalidType)
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            match self.peek_byte()? {
                b'-' => {
                    self.advance();
                    visitor.visit_i16(self.parse_integer(true)?)
                }
                _ => visitor.visit_i16(self.parse_integer(false)?),
            }
        } else {
            Err(Error::InvalidType)
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            match self.peek_byte()? {
                b'-' => {
                    self.advance();
                    visitor.visit_i32(self.parse_integer(true)?)
                }
                _ => visitor.visit_i32(self.parse_integer(false)?),
            }
        } else {
            Err(Error::InvalidType)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            match self.peek_byte()? {
                b'-' => {
                    self.advance();
                    visitor.visit_i64(self.parse_integer(true)?)
                }
                _ => visitor.visit_i64(self.parse_integer(false)?),
            }
        } else {
            Err(Error::InvalidType)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            visitor.visit_u8(self.parse_integer(false)?)
        } else {
            Err(Error::InvalidType)
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            visitor.visit_u16(self.parse_integer(false)?)
        } else {
            Err(Error::InvalidType)
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            visitor.visit_u32(self.parse_integer(false)?)
        } else {
            Err(Error::InvalidType)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let b'i' = self.next_byte()? {
            visitor.visit_u64(self.parse_integer(false)?)
        } else {
            Err(Error::InvalidType)
        }
    }

    serde_if_integer128! {

        fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            if let b'i' = self.next_byte()? {
                visitor.visit_u128(self.parse_integer(false)?)
            } else {
                Err(Error::InvalidType)
            }
        }

        fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            if let b'i' = self.next_byte()? {
                match self.peek_byte()? {
                    b'-' => {
                        self.advance();
                        visitor.visit_i128(self.parse_integer(true)?)
                    }
                    _ => visitor.visit_i128(self.parse_integer(false)?),
                }
            } else {
                Err(Error::InvalidType)
            }
        }
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
        Err(Error::Unsupported("char"))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(core::str::from_utf8(self.parse_byte_string()?)?)
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
        Err(Error::Unsupported("()"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("()")) // TODO: Use the name to provide better errors
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

#[cfg(test)]
mod tests {

    use crate::from_bytes;

    #[test]
    fn zero_lenght_byte_string() {
        assert_eq!("", from_bytes::<&'static str>(b"0:").unwrap())
    }
}
