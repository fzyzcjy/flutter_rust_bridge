# SSE Codec

This codec uses a simple serialization based approach.
For example, all fields of a struct are written to a byte buffer one by one.
The byte array is transferred across the language boundary,
and the other side decodes the fields from the buffer to reconstruct the object.

* The Dart-side buffer owns its Rust allocation until it transfers the raw buffer to Rust.
* Releasing an owned buffer must pass its original allocation length to Rust, even after clearing the Dart-side ownership state.
