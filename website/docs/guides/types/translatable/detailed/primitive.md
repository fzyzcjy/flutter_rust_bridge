# Primitives

Types like `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `isize`, `usize`, `i128`, `u128`
will be converted to `int` or `BigInt` depending on whether they fit in the integer.

In addition, the `#[frb(type_64bit_int)]` can be utilized to force the 64bit types to be integers.

For other types like `f32`, `f64`, `bool`, `String`, please refer to the simple correspondence table for the mapping.
