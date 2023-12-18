# WASM

`flutter_rust_bridge_codegen` can also generate code to run in browsers using
`wasm_bindgen`. To generate a WASM-specific file, pass this option to your invocation:

```shell
flutter_rust_bridge_codegen .. --wasm
```

By default this will create several new files:

```
├── lib
│   ├── ...
│   ├── bridge_generated.io.dart
│   └── bridge_generated.web.dart
└── native/src
    ├── bridge_generated.io.rs
    └── bridge_generated.web.rs
```

The `.io` and `.web` modules implement platform-specific helpers. This
split is mandatory for Dart due to its module system, however if you prefer to keep the Rust bridge in a single file pass the `--inline-rust`
flag as well.

Check out `Integrating with Web` for instructions
on how to consume the web bridge.

have a look at [issue 860](https://github.com/fzyzcjy/flutter_rust_bridge/issues/860)