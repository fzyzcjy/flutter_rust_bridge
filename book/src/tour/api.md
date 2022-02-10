# `native/src/api.rs`

This is where exported functions, structs and enums live. You can import code from
other files, but only those *defined* here will be eligible for code generation.
This limitation is expected to be lifted once [fzyzcjy/flutter_rust_bridge#319](https://github.com/fzyzcjy/flutter_rust_bridge/pull/319) lands.

Furthermore, structs and enums may only comprise of types that are themselves
eligible for code generation.

To review the subset of currently eligible functions and types, see [the example file here](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs).