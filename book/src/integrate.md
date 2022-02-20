# Integrating with existing projects

This guide is an intermediate-level introduction to integrating Rust with
an existing Flutter project. If you are new to Rust or configuring
build processes in general, we suggest looking at [the template tour](template/tour.md)
to learn about the moving parts behind a `flutter run`.

Before following this guide, upgrade your Flutter SDK, and if possible
refresh your native build folders (`android`, `ios`, etc.) to make the process
as straightforward as possible.

**Remark:** Most complexity does *not* come from this library, `flutter_rust_bridge` - it is as same complex as using raw Dart/Flutter FFI with Rust. In other words, it is the Dart/Flutter + Rust toolchain that takes time to set up.

