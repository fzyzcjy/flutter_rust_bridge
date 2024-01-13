# Pure Dart

This page discusses scenarios when you want to use Rust with Dart (without Flutter).
It is totally supported to use Dart without Flutter,
because flutter_rust_bridge does not require anything specific to Flutter.

As a quickstart,
one way is to clone flutter_rust_bridge, and start with
[this minimal example](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/dart_minimal),
and modify according to your needs.
More specifically, to run the demo, you can run

```shell
dart --enable-experiment=native-assets run lib/main.dart
```

in the folder `frb_example/dart_minimal`.
The `--enable-experiment=native-assets` is temporarily needed, but is to be removed when Dart release later versions.

By the way,
[this example](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/pure_dart)
is also pure-Dart (without Flutter),
and contains a lot of tests of this library.

This example currently uses some glue of internal frb_utils to build Rust code.
However, this is just temporary before [the official toolchain](https://github.com/dart-lang/native/issues/883) is released.
Therefore, those temporary glue are not made as a publishable package.
