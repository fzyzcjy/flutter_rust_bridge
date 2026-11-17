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
existing names. Packages should share a compatible FRB Rust runtime dependency.
