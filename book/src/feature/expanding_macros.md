# Expanding macros

This library automatically handles macros inside your code. For example, support you calls a macro that will generate a struct, then even if that struct is not in the code directly, this library can understand it.

The implementation is as follows: To produce code for types or functions that are generated through macros, it is necessary to first expand the code before it is parsed. This is done by invoking [cargo-expand](https://crates.io/crates/cargo-expand), a tool that expands all macros, resulting in code that can then be parsed.

Caution: This expansion process **cannot** be utilized when the code-generator is invoked within a `build.rs` script.
The issue here is that `cargo-expand` triggers a project build, and invoking it within `build.rs` would lead to a deadlock, as `cargo-expand` would wait for the calling cargo build to complete.
In such cases, code is read from files without macro expansion.
If your API definition does not rely on macros for code generation, this works fine.
Otherwise, you have to call the `flutter_rust_bridge_codegen` binary seperately.
