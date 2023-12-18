# `native/src/api.rs`

This is the default entry point for your library. Only functions defined here will be eligible for codegen.
Functions may use types not defined in this file as parameter or return types, but those types must have
been imported through `pub use` so that they are visible from `native/src/bridge_generated.rs`.

Only types defined within the current crate are eligible for codegen.
Furthermore, structs and enums may only comprise of types that are themselves eligible.

To review the subset of currently eligible functions and types, see [the example file here](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs).