# Equals and Hash

This page discusses the `operator==` (equals) and `hashCode` of the automatically generated Dart class.

## Default

The default for non-`freezed` classes is field-by-field comparison.
You can use `#[frb(non_hash, non_eq)]` to disable such generated code.

The default for `freezed` classes: Usually field-by-field comparison (see `freezed`'s doc for more details).

## Custom (arbitrary)

Arbitrary equals/hash function can also be implemented by using the [extra Dart code](../misc-features/dart-code)
feature.
For example, `#[frb(dart_code = "int get hashCode { arbitrary_code_calling_whatever_Rust_and_Dart_things }")]`.
