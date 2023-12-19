# Upgrade to V2

This guide shows how to upgrade a project using flutter_rust_bridge V1 to V2.

One way is as follows:

* Follow the [quickstart](../../quickstart) to integrate V2 boilerplate into the existing project (in a few commands).
* Move original Rust code to `rust/src/api/simple.rs`.
* Run code generator (whenever your code changes).
* Fix compilation errors (usually just a rename or relocation) and runtime exceptions (usually have hints).

Some changes and quick fixes (renames) are:

* `SyncReturn` type becomes annotation: Change `fn f() -> SyncReturn<T> {}` to `#[frb(sync)] fn f() -> T {}`
* `api.functionName()` -> `functionName()`
* `DartAbi` -> `DartDynamic` (simple name alias)
* `ZeroCopyBuffer<T>` -> `T` (just remove the wrapper, it is automatically zero-copy by default)
* Initialize the system via `RustLib.init()` when starting app.
* `flutter_rust_bridge_codegen` -> `flutter_rust_bridge_codegen generate`
* `flutter_rust_bridge_serve` -> `flutter_rust_bridge_codegen build-web` + standard `flutter run` (or run in IDE)

Surely, there are alternative approaches.
For example, if you want to keep the compilation and integration between Rust and Dart,
or like to use command line arguments,
just find the corresponding counterparts in V2 and rename things.

Some flags are removed,
when, for example, they exist mainly for compatibility of later V1 with earlier V1 versions.
If you find the removed flag important for your scenario, feel free to create an issue.
