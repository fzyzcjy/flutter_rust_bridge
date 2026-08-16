# Cargokit

flutter_rust_bridge uses Cargokit for seamless integration of cargo build
with flutter applications and packages.

Cargokit remains the default `flutter_rust_bridge_codegen create/integrate` backend for compatibility with existing projects and older Flutter/Dart SDKs.
The [Native Assets](native-assets) backend is available through `--integration-backend native-assets`.

## Cargokit Details

Please refer to its documentation for how to use it.
The original GitHub repository is https://github.com/irondash/cargokit.
That repository has been archived; flutter_rust_bridge now uses the `feature/swift-package-manager` branch of https://github.com/star4277/cargokit for template and submodule updates.
In addition, it has a companion blog about how to integrate it at https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/.

The following links may also be useful for customizations:

* Configuration: https://github.com/irondash/cargokit/blob/main/docs/architecture.md#configuring-cargokit
* Use precompiled binaries (instead of default compile-on-the-fly): https://github.com/irondash/cargokit/blob/main/docs/precompiled_binaries.md

## Swift Package Manager

The generated iOS and macOS Cargokit templates support both Swift Package Manager and CocoaPods. The existing podspecs remain available for projects that use CocoaPods.

Swift Package Manager consumes a local Rust dynamic framework XCFramework. During package manifest evaluation, each generated `Package.swift` runs `cargokit/build_spm.sh` before declaring the XCFramework as a binary target. Cargokit selects debug or release mode from the Xcode build environment and rebuilds changed Rust artifacts automatically.

The XCFramework is generated next to the corresponding `Package.swift` and is ignored by Git. A failed Cargokit build stops Swift package resolution with the script's exit status instead of leaving a missing binary target.

For new projects that can require a recent Flutter/Dart SDK, consider the [Native Assets](native-assets) backend.
