# Initialization

## Use `#[frb(init)]`

If you want to initialize the Rust side during Flutter/Dart startup,
just use `#[frb(init)]` annotation on functions.

For example:

```rust
#[frb(init)]
pub fn lets_init_app_here() {
    // ...
}
```

Indeed, the `flutter_rust_bridge_codegen create` already creates an init function, `init_app`, for you.

Remark: The function needs to be inside your Rust input folder, otherwise it is simply ignored.

## Use `#[frb(init_dart_code = ...)]`

If a Rust declaration needs Dart-side initialization during `RustLib.init()`,
annotate the generated Rust function with `#[frb(init_dart_code = "...")]`.
The Dart code runs after all `#[frb(init)]` Rust initializers, inside the generated
`executeRustInitializers` method, where `api` refers to the generated Dart API.

For example:

```rust
#[frb(init_dart_code = r#"
api.setupDartRuntime();
"#)]
pub fn setup_dart_runtime() {}
```

## Alternative approach

Alternatively, if you do not want to use the annotation above,
just simply call arbitrary Rust function as you like.
There is nothing special about initialization compared to normal execution.

For example:

```dart
Future<void> main() async {
  await RustLib.init();
  await myRustInitLogic(); // or `sync` if you like
  // ...
}
```

```rust
fn my_rust_init_logic() {
    // initialize whatever things here
}
```
