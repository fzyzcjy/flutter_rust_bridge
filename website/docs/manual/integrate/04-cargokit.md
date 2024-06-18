# Flutter package via Cargokit

In this page, we show how to use [Cargokit](https://github.com/irondash/cargokit)
to create a Flutter package that can be published and used by other Flutter apps/packages.

## Step 1: Follow Cargokit tutorial

For simplicity, we directly clone it to get a Flutter package that can compile Rust code:

```shell
git clone https://github.com/irondash/hello_rust_ffi_plugin && cd hello_rust_ffi_plugin
flutter pub get
```

<details>
<summary>The following links may also be useful for customizations (click to expand).</summary>

* Tutorial (step-by-step generating that sample repo): https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
* Configuration: https://github.com/irondash/cargokit/blob/main/docs/architecture.md#configuring-cargokit
* Use precompiled binaries (instead of default compile-on-the-fly): https://github.com/irondash/cargokit/blob/main/docs/precompiled_binaries.md

</details>

## Step 2: Add `flutter_rust_bridge`

Create `flutter_rust_bridge.yaml` with the following content:

```yaml
rust_root: rust/
rust_input: crate::api
dart_output: lib/src/rust
```

Let's put a few boilerplate files there as well (feel free to put whatever you like instead):

```shell
mkdir -p lib/src/rust rust/src/api
echo "pub mod simple;" > rust/src/api/mod.rs
wget -O rust/src/api/simple.rs https://raw.githubusercontent.com/fzyzcjy/flutter_rust_bridge/master/frb_example/flutter_via_create/rust/src/api/simple.rs
```

Then we can run code generator:

```shell
flutter_rust_bridge_codegen generate
```

## Remarks

[It seems that](https://github.com/irondash/cargokit/issues/39#issuecomment-1831584430),
after Dart [native assets](native-assets) is stablized,
cargokit will also utilize it.
