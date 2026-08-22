# Migrate from Cargokit to Native Assets

Native Assets changes how the Rust library is built and bundled; the Rust and Dart APIs stay unchanged.

## Migrate

1. Run the integration command from the Flutter package root:

   ```bash
   flutter_rust_bridge_codegen integrate --integration-backend native-assets
   ```

2. Check the generated `hook/build.dart` and the [Rust crate requirements](native-assets#rust-crate-requirements).

3. Remove the old Cargokit integration:

   - **Apps**: Remove `rust_builder/`, its `pubspec.yaml` path dependency, and its `analysis_options.yaml` exclusion.
   - **Packages**: Remove `cargokit/` and the Flutter plugin entries used only by the old `ffiPlugin` scaffold.

4. Refresh dependencies and regenerate the bridge:

   ```bash
   flutter pub get
   flutter_rust_bridge_codegen generate
   ```

Keep the Rust crate, generated bridge code, and your own Dart and Rust code. Test every platform supported by the project after migrating.
