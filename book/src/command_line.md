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

You can also run `flutter_rust_bridge_codegen` with no arguments, provided one of these files exist (in order of priority):

- `.flutter_rust_bridge.yml`
- `.flutter_rust_bridge.yaml`
- `.flutter_rust_bridge.json`

The codegen will try to read a configuration from any of these files. The same arguments from the CLI are accepted, but
they will be in snake_case.