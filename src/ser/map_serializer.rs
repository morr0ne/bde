use std::collections::BTreeMap;

use crate::{value::ValueSerializer, ByteString, Dictionary, Error, Result};
use serde::ser::{self, Impossible, Serialize, SerializeMap};

use super::Serializer;

pub struct MapSerializer<'a, W: 'a> {
    serializer: &'a mut Serializer<W>,
    dictionary: Dictionary,
}

impl<'a, W> MapSerializer<'a, W> {
    pub fn new(serializer: &'a mut Serializer<W>) -> Self {
        Self {
            serializer,
            dictionary: BTreeMap::new(),
        }
    }
}

pub struct MapKeySerializer;

impl MapKeySerializer {
    pub const fn new() -> Self {
        Self {}
    }
}

impl ser::Serializer for MapKeySerializer {
    type Ok = ByteString;

    type Error = Error;

    type SerializeSeq = Impossible<ByteString, Error>;

    type SerializeTuple = Impossible<ByteString, Error>;

    type SerializeTupleStruct = Impossible<ByteString, Error>;

    type SerializeTupleVariant = Impossible<ByteString, Error>;

    type SerializeMap = Impossible<ByteString, Error>;

    type SerializeStruct = Impossible<ByteString, Error>;

    type SerializeStructVariant = Impossible<ByteString, Error>;

    fn serialize_str(self, value: &str) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(ByteString::from(value))
    }

    fn serialize_bytes(self, value: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        Ok(ByteString::from(value))
    }

    fn serialize_bool(self, _value: bool) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i8(self, _value: i8) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i16(self, _value: i16) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i32(self, _value: i32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i64(self, _value: i64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u8(self, _value: u8) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u16(self, _value: u16) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u32(self, _value: u32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u64(self, _value: u64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_f32(self, _value: f32) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_f64(self, _value: f64) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_char(self, _value: char) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_seq(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_tuple(self, _len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::MapKeyMustBeByteString)
    }
}

impl<'a, W> ser::SerializeMap for MapSerializer<'a, W>
where
    W: std::io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unreachable!()
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = key.serialize(MapKeySerializer::new())?;
        let value = value.serialize(ValueSerializer::new())?;

        self.dictionary.insert(key, value);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        for (key, value) in self.dictionary {
            key.serialize(&mut *self.serializer)?;
            value.serialize(&mut *self.serializer)?;
        }

        self.serializer.writer.write_all(b"e")?;

        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for MapSerializer<'a, W>
where
    W: std::io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.serialize_entry(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}
