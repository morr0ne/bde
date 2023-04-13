use serde::ser::{self, Impossible, Serialize};

use crate::{ByteString, Error, Result};

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

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        Ok(ByteString::from(value))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        Ok(ByteString::from(value))
    }

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
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
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::MapKeyMustBeByteString)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::MapKeyMustBeByteString)
    }
}
