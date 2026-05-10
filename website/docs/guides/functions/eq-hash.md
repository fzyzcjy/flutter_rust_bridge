# Equals and Hash

This page discusses the `operator==` (equals) and `hashCode` of the automatically generated Dart class.

## Default

The default for non-`freezed` classes is field-by-field comparison.
For Dart collection fields such as `List`, `Map`, and `Set`, this follows Dart's default collection semantics, i.e. identity equality.
You can use `#[frb(non_hash, non_eq)]` to disable such generated code.

The default for `freezed` classes: Usually field-by-field comparison (see `freezed`'s doc for more details).

## Deep collection equality

Set `dart_collection_deep_equality: true` in `flutter_rust_bridge.yaml`, or pass `--dart-collection-deep-equality` to the codegen command, to generate deep equality and hash code for Dart collection fields in non-`freezed` classes.
This affects `List`, `Map`, `Set`, and optional values wrapping those collection types.

To opt in for only one non-`freezed` struct, use `#[frb(dart_collection_deep_equality)]` on that struct instead of enabling the global option.
This does not change the default behavior for other structs in the project.

If you want richer Dart value-class semantics, another option is to use `freezed`, which can also generate deep equality for collection fields.

## Custom (arbitrary)

Arbitrary equals/hash function can also be implemented by using the [extra Dart code](../misc-features/dart-code)
feature.
For example, `#[frb(dart_code = "int get hashCode { arbitrary_code_calling_whatever_Rust_and_Dart_things }")]`.
