# Compatibility and versioning

Since flutter_rust_bridge v2, we will try our best to follow [semantics versioning](https://semver.org/)
with the following exceptions.

Firstly, all flutter_rust_bridge-related packages will need to have exactly the same version. They are:

* Dart package `flutter_rust_bridge` (runtime support)
* Rust package `flutter_rust_bridge` (runtime support)
* Rust package `flutter_rust_bridge_codegen` (code generator)
* Rust package `flutter_rust_bridge_macros` (macros for code generator)

This is because the generated code is working with the runtime support library tightly,
so it is quite hard to not introduce any breaking change between the auto-generated code and the runtime library.
In addition, it seems better to put the limited resources to more important things (e.g. new features) than ensuring this.
[As is pointed out](https://github.com/fzyzcjy/flutter_rust_bridge/issues/1432#issuecomment-1849199332),
this is not very uncommon, e.g. Apple's Safari is kept in sync with MacOS version,
and this should not be a problem with upgrade tools.

(WIP: (1) Make a tool to check these versions agree at runtime or as a doctor command (2) tool to upgrade them in one command.)

Secondly, some classes with comments explicitly mentioning breaking-change things,
especially the `Handler` classes, may contain breaking change in minor version bump.
This is because, firstly, the `Handler` contains a lot of generic arguments and implementation details,
so it is hard to avoid breaking changes (I do not want to bump to flutter_rust_bridge v10 quickly!).
Secondly, such change usually results in compile-time error.
Therefore, you can trivially realize it is a breaking change, without introducing any subtle bugs.
Thirdly, it is not very frequently needed to customize `Handler` itself
(e.g. customizing the `ErrorHandler` is often enough).
But even for this, we will try our best to make it happen as rare as possible.
