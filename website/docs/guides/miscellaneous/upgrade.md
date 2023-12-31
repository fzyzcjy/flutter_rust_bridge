# Upgrade to V2

This guide shows how to upgrade a project using flutter_rust_bridge V1 to V2.

One way is as follows:

* Follow the [quickstart](../../quickstart) to integrate V2 boilerplate into the existing project (in a few commands). Usually, you just need to run `flutter_rust_bridge_codegen integrate`.
* Move original Rust code (e.g. in `api.rs`) to `rust/src/api/simple.rs`. (You can also split into multiple files, e.g. `apple.rs`, `orange.rs`, etc.)
* Run code generator and watch for code changes by running `flutter_rust_bridge_codegen generate --watch`.
* Fix compilation errors (usually just a rename or relocation) and runtime exceptions (usually have hints).
* Clean up old files and generated artifacts
    * **Tear down automatic build**: If you followed fzyzcjy's [StackOverflow answer](https://stackoverflow.com/a/69515060/6798201) on automatic integration of Flutter and Rust, you also need to tear down the relevant configurations carefully. For the iOS setup, you need to reverse the setup steps you did by either removing the old Build Phase (if using method 1) or deleting the script_phase in your `Podfile` (if using method 2). For the Android setup, you need to reverse your changes to the `build.gradle` file.
    * **Android-specific step**: Delete the `android/app/src/main/jniLibs/` folder, if this is where the old binaries are stored.
    * **iOS-specific steps**:
        1. Clean up `bridge_generated`:
            * Delete the `ios/Runner/bridge_generated.h` file
            * Remove the line `#import "bridge_generated.h"` in `ios/Runner/Runner-Bridging-Header.h`
            * Remove `bridge_generated.h` from the "Copy Bundle Resources" build phase

        2. Delete the `ios/Runner/libmyapp.a` and remove it from the "Link Binary With Libraries" build phase.

        3. Remove the `print("dummy_value=\(dummy_method_to_enforce_bundling())");` line in `ios/Runner/AppDelegate.swift` if you had that workaround.

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
