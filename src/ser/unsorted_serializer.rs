use serde::{ser, Serialize};
use std::io::Write;

use crate::{Error, Result};

use super::map_serializer::UnsortedMapSerializer;

pub struct UnsortedSerializer<W> {
    writer: W,
}

impl<W> UnsortedSerializer<W>
where
    W: Write + Sized,
{
    pub const fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Write for UnsortedSerializer<W>
where
    W: Write + Sized,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<'a, W> ser::Serializer for &'a mut UnsortedSerializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = UnsortedMapSerializer<'a, W>;
    type SerializeStruct = UnsortedMapSerializer<'a, W>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        Err(Error::Unsupported("bool"))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        write!(self, "i{value}e")?;
        Ok(())
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        Err(Error::Unsupported("f32"))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok> {
        Err(Error::Unsupported("f64"))
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok> {
        Err(Error::Unsupported("char"))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        write!(self, "{0}:{value}", value.len())?;
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        write!(self, "{0}:", value.len())?;
        self.writer.write_all(value)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::Unsupported("None"))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::Unsupported("unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
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
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.write_all(b"l")?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(None)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_seq(None)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_all(b"d")?;
        Ok(UnsortedMapSerializer::new(self))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(None) // There is no reason to pass along the len since we are using a BTreeSet
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a, W> ser::SerializeSeq for &'a mut UnsortedSerializer<W>
where
    W: std::io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_all(b"e")?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut UnsortedSerializer<W>
where
    W: std::io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_all(b"e")?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleStruct for &'a mut UnsortedSerializer<W>
where
    W: std::io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.write_all(b"e")?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut UnsortedSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut UnsortedSerializer<W>
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
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write_all(b"e")?;
        Ok(())
    }
}
