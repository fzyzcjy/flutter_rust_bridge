# Command line arguments

Simply add `--help` to see full documentation. The following is a snapshot when running the command with `--help`:

```shell
$ flutter_rust_bridge_codegen --help
{{#include help.txt}}
```

## `flutter_rust_bridge_serve`

```shell
$ dart run flutter_rust_bridge_serve --help
{{#include help.serve.txt}}
```

## Configuration files

You can run `flutter_rust_bridge_codegen` with no arguments, provided any of these files exists in the working directory (in order of priority):

- `.flutter_rust_bridge.yml`
- `.flutter_rust_bridge.yaml`
- `.flutter_rust_bridge.json`

The codegen will try to read a configuration from these files. Otherwise, you can pass to the CLI any YAML file that contains the config.
The same arguments from the CLI are accepted, but they will be in snake_case.

```yaml
# in .flutter_rust_bridge.yml
rust_input:
  - path/to/api.rs
dart_output:
  - path/to/bridge_generated.dart
```

Similarly, if you're calling `flutter_rust_bridge_codegen` from the root of your Dart project, you can also fill in your config
under the `flutter_rust_bridge` entry in `pubspec.yaml`:

```yaml
# put this somewhere in your pubspec.yaml
flutter_rust_bridge:
  rust_input:
    - path/to/api.rs
  dart_output:
    - lib/src/bridge_generated.dart
```