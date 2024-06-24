# Provide parameters

It is equivalent to provide a parameter using the three approaches below.

In addition, you can provide configs by both configuration file / pubspec.yaml and command line at the same time.
The latter takes precedence when in conflict.

## Configuration file

The suggested mode to run `flutter_rust_bridge_codegen`,
is to create `flutter_rust_bridge.yaml` configuration file,
and invoke command line without arguments.

All these file names work
(though I personally suggest to use `flutter_rust_bridge.yaml`,
because it matches the Flutter convention, the "`pubspec.yaml`"):

<details>
<summary>Full name list</summary>

- `.flutter_rust_bridge.yml`
- `.flutter_rust_bridge.yaml`
- `.flutter_rust_bridge.json`
- `flutter_rust_bridge.yml`
- `flutter_rust_bridge.yaml` (suggested)
- `flutter_rust_bridge.json`

</details>

The `flutter_rust_bridge_codegen create/integrate` already creates a configuration file for you,
which is often sufficient.

The parameter names are the same as the one shown in command line help,
except that there are no `no_` negations here.
You can also refer to the fields of
[this struct](https://docs.rs/flutter_rust_bridge_codegen/latest/lib_flutter_rust_bridge_codegen/codegen/struct.Config.html)
for parameter names.
Anyway, if a wrong name is provided, it will be immediately recognized and reported.

## Command line arguments

For example, suppose you want to provide `rust_input` argument,
then just do `--rust-input your_value`.

## Inside `pubspec.yaml`

Similarly, if you're calling `flutter_rust_bridge_codegen` from the root of your Dart project,
you can also fill in your config under the `flutter_rust_bridge` entry in `pubspec.yaml`:

```yaml
# put this somewhere in your pubspec.yaml
flutter_rust_bridge:
  rust_input: crate::api
  rust_root: rust/
  dart_output: lib/src/rust
  # ...
```

