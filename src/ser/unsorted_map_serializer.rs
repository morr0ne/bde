use std::io::Write;

use crate::{Error, Result};
use serde::ser::{self, Serialize};

use super::{map_key_serializer::MapKeySerializer, UnsortedSerializer};

pub struct UnsortedMapSerializer<'a, W: 'a> {
    pub serializer: &'a mut UnsortedSerializer<W>,
}

impl<'a, W> UnsortedMapSerializer<'a, W> {
    pub fn new(serializer: &'a mut UnsortedSerializer<W>) -> Self {
        Self { serializer }
    }
}

impl<'a, W> ser::SerializeMap for UnsortedMapSerializer<'a, W>
where
    W: std::io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = key.serialize(MapKeySerializer::new())?;
        key.serialize(&mut *self.serializer)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.serializer.write_all(b"e")?;

        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for UnsortedMapSerializer<'a, W>
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
        key.serialize(&mut *self.serializer)?;
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.write_all(b"e")?;

        Ok(())
    }
}
