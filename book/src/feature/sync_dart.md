# Synchronous in Dart

If you need to generate synchronous functions in Dart, you can use `SyncReturn<T>` as the return type.

We suggest only do this for very quick Rust functions, or the Dart UI will be blocked.

Currently, `SyncReturn` supports `String`, `Vec<u8>` and primitive types(`bool`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`). Please open an issue if you need other types.