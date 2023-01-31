use serde::{ser, Serialize};
use std::io::Write;

use crate::Error;

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

    fn serialize_bool(self, value: bool) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_bool(value)
    }

    fn serialize_i8(self, value: i8) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_i8(value)
    }

    fn serialize_i16(self, value: i16) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_i16(value)
    }

    fn serialize_i32(self, value: i32) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_i32(value)
    }

    fn serialize_i64(self, value: i64) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_i64(value)
    }

    fn serialize_u8(self, value: u8) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_u8(value)
    }

    fn serialize_u16(self, value: u16) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_u16(value)
    }

    fn serialize_u32(self, value: u32) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_u32(value)
    }

    fn serialize_u64(self, value: u64) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_u64(value)
    }

    fn serialize_f32(self, value: f32) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_f32(value)
    }

    fn serialize_f64(self, value: f64) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_f64(value)
    }

    fn serialize_char(self, value: char) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_char(value)
    }

    fn serialize_str(self, value: &str) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_bytes(value)
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_none()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.unsorted_serializer.serialize_some(value)
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_unit()
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        self.unsorted_serializer
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
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
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.unsorted_serializer
            .serialize_newtype_variant(name, variant_index, variant, value)
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        self.unsorted_serializer.serialize_seq(len)
    }

    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        self.unsorted_serializer.serialize_tuple(len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        self.unsorted_serializer.serialize_tuple_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        self.unsorted_serializer
            .serialize_tuple_variant(name, variant_index, variant, len)
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        self.write_all(b"d")?;
        Ok(MapSerializer::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(None) // There is no reason to pass along the len since we are using a BTreeSet
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        self.unsorted_serializer
            .serialize_struct_variant(name, variant_index, variant, len)
    }
}
