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

If a Rust declaration or macro needs Dart-side initialization during
`RustLib.init()`, annotate a generated Rust function with
`#[frb(init_dart_code = "...")]`.

This is different from `#[frb(init)]`:

* `#[frb(init)]` calls a Rust function during startup.
* `#[frb(init_dart_code = ...)]` inserts Dart code into the generated startup
  flow.

The Dart code runs after all `#[frb(init)]` Rust initializers, inside the
generated `executeRustInitializers` method. In that context, `api` refers to the
generated `RustLibApi` object. This is the raw generated API used by
`RustLib.instance.api`, not the shorter public wrapper functions in `api/*.dart`.

For example:

```rust
#[frb(init_dart_code = r#"
print("hi");
"#)]
pub fn setup_dart_runtime() {}
```

The generated code will look roughly like this:

```dart
@override
Future<void> executeRustInitializers() async {
  await api.letsInitAppHere();
  print("hi");
}
```

Multiple `init_dart_code` attributes are supported. They are emitted in the same
order as the corresponding generated Rust functions are parsed.

The snippet is copied as Dart code. Therefore it must be valid Dart statement
code in the body of an async method; codegen does not type-check or otherwise
understand the snippet before inserting it.

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
