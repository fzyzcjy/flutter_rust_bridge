# WASM

`flutter_rust_bridge_codegen` can also generate code to run in browsers using
`wasm_bindgen`. To generate WASM-specifc files, pass in these two options to your
invocation:

```shell
flutter_rust_bridge_codegen .. --wasm --dart-decl-output <DECL>
```

where `DECL` is the path to the common class/function declarations file.
For example, if you emit your Dart bridge to `lib/bridge_generated.dart`,
you can put the declarations file at `lib/bridge_definitions.dart`

By default this will create several new files:

```
├── lib
│   ├── bridge_definitions.dart
│   ├── bridge_generated.io.dart
│   └── bridge_generated.web.dart
└── native/src
    ├── bridge_generated.io.rs
    └── bridge_generated.web.rs
```

The `.io` and `.web` modules implement platform-specific helpers. This
split is mandatory for Dart due to its module system, however if you prefer to keep the Rust bridge in a single file pass the `--inline-rust`
flag as well.

Check out [Integrating with Web](../integrate/web.md) for instructions
on how to consume the web bridge.
