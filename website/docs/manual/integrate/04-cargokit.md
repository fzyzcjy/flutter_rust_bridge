# Flutter package via Cargokit

In order to use [Cargokit](https://github.com/irondash/cargokit)
to create a Flutter package that can be published and used by other Flutter apps/packages,
please refer to the demo in https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/flutter_package.
(Remember to modify the local dependencies in `Cargo.toml`, `pubspec.yaml` and `flutter_rust_bridge.yaml` to non-local.)

## Remarks

[It seems that](https://github.com/irondash/cargokit/issues/39#issuecomment-1831584430),
after Dart [native assets](native-assets) is stablized,
cargokit will also utilize it.
