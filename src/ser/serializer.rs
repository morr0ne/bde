use serde::{ser, Serialize};
use std::io::Write;

use crate::{Error, Result};

use super::{map_serializer::MapSerializer, unsorted_serializer::UnsortedSerializer};

pub struct Serializer<W> {
    unsorted_serializer: UnsortedSerializer<W>,
}

impl<W> Serializer<W>
where
    W: Write + Sized,
{
    pub const fn new(writer: W) -> Self {
        Self {
            unsorted_serializer: UnsortedSerializer::new(writer),
        }
    }
}

impl<W> Write for Serializer<W>
where
    W: Write + Sized,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.unsorted_serializer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.unsorted_serializer.flush()
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();

    type Error = Error;
    type SerializeSeq = &'a mut UnsortedSerializer<W>;
    type SerializeTuple = &'a mut UnsortedSerializer<W>;
    type SerializeTupleStruct = &'a mut UnsortedSerializer<W>;
    type SerializeTupleVariant = &'a mut UnsortedSerializer<W>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MapSerializer<'a, W>;
    type SerializeStructVariant = &'a mut UnsortedSerializer<W>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_bool(value)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_i8(value)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_i16(value)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_i32(value)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_i64(value)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_u8(value)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_u16(value)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_u32(value)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_u64(value)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_f32(value)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_f64(value)
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_char(value)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_bytes(value)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.unsorted_serializer.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.unsorted_serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.unsorted_serializer
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.unsorted_serializer
            .serialize_newtype_struct(name, value)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        self.unsorted_serializer
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.unsorted_serializer.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.unsorted_serializer.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.unsorted_serializer.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.unsorted_serializer
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_all(b"d")?;
        Ok(MapSerializer::new(self))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(None) // There is no reason to pass along the len since we are using a BTreeSet
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.unsorted_serializer
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}
