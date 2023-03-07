use serde::Deserialize;

use crate::Result;

mod deserializer;

pub use deserializer::Deserializer;

/// # Errors
/// TODO
pub fn from_bytes<'a, T>(bytes: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(bytes);
    let value = T::deserialize(&mut deserializer)?;
    deserializer.check_trailing_bytes()?;
    Ok(value)
}
