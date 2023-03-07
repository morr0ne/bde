# 0.3.0 (Unreleased)

### Changed
- `ByteString::into_vec` to `ByteString::inner`

### Removed
- Removed undocumented methods on ByteString

# 0.2.2 (February 6, 2023)

### Added
- `ValueSerializer` now serializes structs

### Fixed
- `serialize_tuple_struct` is now properly implemented


# 0.2.1 (February 1, 2023)

### Fixed
- Accidental breakage of MapAccess implementation


# 0.2.0 (February 1, 2023)

### Added
- `ByteString` is now marked with repr(transparent)
- Implemented `Deserializer` for `Value`
- Added `from_value` and `to_value` helper functions
- Added `to_bytes_unsorted` and `to_writer_unsorted` helper functions
- Added `UnsortedSerializer`

### Removed
- `bool` and `char` support since it's not actually part of the spec.

### Changed
- Marked some functions as const
- `ByteString::from` is now implementated properly using traits
- Renamed `Serializer` to `ValueSerializer`
- Made `value` module private again
- The `Deserializer` now throws an error when encountering unsorted keys


# 0.1.1 (January 29, 2023)

### Added
- `Dictionary` type alias to avoid repeating BTreeMap a bunch of times