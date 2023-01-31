use serde::ser::Serialize;

use crate::Result;

pub(crate) mod map_key_serializer;
mod map_serializer;
mod serializer;
mod unsorted_map_serializer;
mod unsorted_serializer;

pub use serializer::Serializer;
pub use unsorted_serializer::UnsortedSerializer;

pub fn to_writer_unsorted<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = UnsortedSerializer::new(writer);
    value.serialize(&mut ser)
}

pub fn to_bytes_unsorted<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_unsorted(&mut writer, value)?;
    Ok(writer)
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: std::io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}
