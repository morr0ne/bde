/*
The current code was adapted from the serde_bytes witch contains the following copyright notice.


Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
*/

use core::{
    cmp::{self, Ordering},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};
use std::{
    convert::Infallible,
    ops::{Index, IndexMut},
    slice::SliceIndex,
    str::FromStr,
    string::String,
    vec::Vec,
};

use serde::{
    de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
    ser::{Serialize, Serializer},
};

/// Wrapper around `Vec<u8>` to serialize and deserialize efficiently.
/// see <https://github.com/serde-rs/bytes>
#[derive(Clone, Default, Eq, Ord)]
#[repr(transparent)]
pub struct ByteString {
    inner: Vec<u8>,
}

impl ByteString {
    /// Construct a new, empty `ByteString`.
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Construct a new, empty `ByteString` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }

    /// Wraps an existing `Vec` into a `ByteString`
    pub const fn from_vec(vec: Vec<u8>) -> Self {
        Self { inner: vec }
    }

    /// Return the inner vector
    pub fn inner(self) -> Vec<u8> {
        self.inner
    }
}

impl Debug for ByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&String::from_utf8_lossy(self))
    }
}

impl<T: Into<Vec<u8>>> From<T> for ByteString {
    fn from(vec: T) -> Self {
        Self::from_vec(vec.into())
    }
}

impl AsRef<[u8]> for ByteString {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl AsMut<[u8]> for ByteString {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl Deref for ByteString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ByteString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<Rhs> PartialEq<Rhs> for ByteString
where
    Rhs: ?Sized + AsRef<[u8]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<Rhs> PartialOrd<Rhs> for ByteString
where
    Rhs: ?Sized + AsRef<[u8]>,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Hash for ByteString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl IntoIterator for ByteString {
    type Item = u8;
    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a ByteString {
    type Item = &'a u8;
    type IntoIter = <&'a [u8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut ByteString {
    type Item = &'a mut u8;
    type IntoIter = <&'a mut [u8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for ByteString {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.inner.index(index)
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for ByteString {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.inner.index_mut(index)
    }
}

impl FromStr for ByteString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Serialize for ByteString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.inner)
    }
}

struct ByteStringVisitor;

impl<'de> Visitor<'de> for ByteStringVisitor {
    type Value = ByteString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("byte string")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<ByteString, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let len = cmp::min(visitor.size_hint().unwrap_or(0), 4096);
        let mut bytes = Vec::with_capacity(len);

        while let Some(b) = visitor.next_element()? {
            bytes.push(b);
        }

        Ok(ByteString::from(bytes))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<ByteString, E>
    where
        E: Error,
    {
        Ok(ByteString::from(v))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<ByteString, E>
    where
        E: Error,
    {
        Ok(ByteString::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<ByteString, E>
    where
        E: Error,
    {
        Ok(ByteString::from(v))
    }

    fn visit_string<E>(self, v: String) -> Result<ByteString, E>
    where
        E: Error,
    {
        Ok(ByteString::from(v))
    }
}

impl<'de> Deserialize<'de> for ByteString {
    fn deserialize<D>(deserializer: D) -> Result<ByteString, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_byte_buf(ByteStringVisitor)
    }
}
