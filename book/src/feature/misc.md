# Miscellaneous

## Separate generated definitions from implementations

The generated `bridge_generated.dart` by default contains definitions of the APIs as well as the implementations. With the flag `--dart-decl-output`, the two can be separated, and the definitions will not contain anything like `dart:ffi`.

More information: [#298](https://github.com/fzyzcjy/flutter_rust_bridge/issues/298).