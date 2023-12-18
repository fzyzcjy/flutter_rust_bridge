# Zero copy

By default, copies are avoided automatically when possible,
so there is no manual operations needed to have this feature.

Due to limitation of Dart VM APIs, it is not possible to zero-copy in all scenarios,
since it is the 
["external typed data"](https://github.com/dart-lang/sdk/blob/6fcd15c1aa024bd42056487374a146be492277a2/runtime/include/dart_native_api.h#L93)
when calling Dart VM's [`Dart_PostCObject`](https://github.com/dart-lang/sdk/blob/6fcd15c1aa024bd42056487374a146be492277a2/runtime/include/dart_native_api.h#L127)
that allows zero copy.
Therefore, when you are sending `Vec<u8>` (or `Vec<i8>` or friends) from Rust to Dart
using asynchronous Dart mode or streaming in Android/iOS/Windows/MacOS/Linux, it automatically works.

If you want to make code extra clear that zero copy is utilized,
you can use the `ZeroCopyBuffer<_>` type (e.g. `ZeroCopyBuffer<Vec<u8>>`).
