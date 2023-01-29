# Unreleased

### Added
- `ByteString` is now marked with repr(transparent)

### Removed
- bool support since it's not actually part of the spec.

### Changed
- Marked some functions as const
- `ByteString::from` is now implementated properly using traits
- Renamed `Serializer` to `ValueSerializer`
- Made `value` module private again


# 0.1.1 (January 29, 2023)

### Added
- `Dictionary` type alias to avoid repeating BTreeMap a bunch of times