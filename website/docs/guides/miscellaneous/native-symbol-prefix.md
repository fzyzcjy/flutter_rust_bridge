# Linking multiple FRB packages into one native binary

When statically linking multiple packages with generated FRB bindings, assign a
unique prefix to each package in its `flutter_rust_bridge.yaml`:

```yaml
native_symbol_prefix: package_a_
```

The equivalent CLI option is `--native-symbol-prefix package_a_`.
Regenerate both Rust and Dart bindings together. The prefix applies to the
package content hash, primary and synchronous dispatchers, and Dart callback
output delivery. Shared FRB runtime exports retain their names.

The default is an empty prefix, preserving existing exports. Use ASCII letters,
digits and underscores. Prefixes must be unique within the final binary. On
Windows, include the prefixed names in any explicit executable export list.
This option affects native exports only; separate web modules retain their
existing names.

## Keep the exports when linking

When a secondary FRB package is linked as a Rust `rlib` and only Dart calls its
exports, the native linker can omit them. In that case, `RustLib.init()` can fail
with `Failed to lookup symbol 'package_a_frb_get_rust_content_hash'`.

Keep the exports reachable from the final binary and visible to Dart. For
example, the [`native_symbol_prefix` fixture](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/native_symbol_prefix/rust/src/lib.rs)
provides a `link_bridge()` function that passes references to its four exports
through `std::hint::black_box`. Call such a function from Rust code that is
retained in the final binary. Alternatively, use your linker's retention options,
such as `-force_load` on Apple platforms, `--whole-archive` on GNU-style linkers,
or `/INCLUDE:<symbol>` on MSVC.

When both packages are linked into the same library, pass the same
`ExternalLibrary` to both generated entrypoints. For example, with `package_a`
and `package_b` as import aliases and `library` as the loaded library:

```dart
await package_a.RustLib.init(externalLibrary: library);
await package_b.RustLib.init(externalLibrary: library);
```

## Share one runtime

The final binary must contain one resolved build each of `flutter_rust_bridge`
and `allo-isolate`. Use a single Cargo resolution with unified features so all
packages link against the same builds. Multiple builds can cause duplicate-symbol
errors at link time for the shared runtime exports, which the prefix does not rename.

Rust-to-Dart logging is shared across entrypoints. Disposing one `RustLib`
entrypoint also tears down the shared logging subscription, even if another
entrypoint is still in use.
