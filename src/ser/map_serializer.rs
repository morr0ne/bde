use serde::ser::{self, Serialize, SerializeMap};
use std::{collections::BTreeMap, io::Write};

use crate::{value::ValueSerializer, Dictionary, Error, Result};

use super::{map_key_serializer::MapKeySerializer, Serializer, UnsortedSerializer};

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

        self.serializer.write_all(b"e")?;

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
