# Miscellaneous features

## Separate generated definitions from implementations

The generated `bridge_generated.dart` by default contains definitions of the APIs as well as the implementations. With the flag `--dart-decl-output`, the two can be separated, and the definitions will not contain anything like `dart:ffi`.

A command example as follow:

```shell
flutter_rust_bridge_codegen .. --dart-decl-output <DECL>
```

where `DECL` is the path to the common class/function declarations file.
For example, if you emit your Dart bridge to `lib/bridge_generated.dart`,
you can put the declarations file at `lib/bridge_definitions.dart`

By default this will create new file:

```
├── lib
│   ├── bridge_definitions.dart
```

More information: [#298](https://github.com/fzyzcjy/flutter_rust_bridge/issues/298).

## Logging for developers

For developers who want to contribute to this project, here is the feature logging that needs to mention.

When the code in `frb_codegen` is modified, usually developers want to build and run it locally for testing. Now with the `init_logger` in `logs.rs` from `frb_codegen`, it is easy to do so.  Take `frb_example/pure_dart` as an example, in `./rust/build.rs`, with:

```rust
use lib_flutter_rust_bridge_codegen::init_logger;
fn main() {
    init_logger("./logs/").unwrap();
...
}
```

Then, all information from standard panic, `log::info!()`,` log::debug()!`...  of `frb_codegen` would be recorded to  `./logs/` with a file name of date, like `2023-02-01.log` in `frb_example/pure_dart/rust` as long as the example is built through `build.rs`. Note, the data from the same day would be appended to the same file.

Moreover, if rust-analyzer is used, then whenever `frb_codegen` is modified, all examples with `build.rs` would be automatically triggered to rebuild. Then the log would be updated automatically to disk, which makes the whole developing routine easier.
