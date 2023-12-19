# Use BrickHub to add to existing projects

This guide is an intermediate-level introduction to integrating Rust with
an existing Flutter project. If you are new to Rust or configuring
build processes in general, we suggest looking at [the template tour](template/tour)
to learn about the moving parts behind a `flutter run`.

Before following this guide, upgrade your Flutter SDK, and if possible
refresh your native build folders (`android`, `ios`, etc.) to make the process
as straightforward as possible.

**Remark:** Most complexity does *not* come from this library, `flutter_rust_bridge` - it is as same complex as using raw Dart/Flutter FFI with Rust. In other words, it is the Dart/Flutter + Rust toolchain that takes time to set up.

## Using the `flutter_rust_bridge` brick

The following sections cover how to set up Rust support from scratch for completeness' sake,
however for your convenience you can also use the [`fluttter_rust_bridge` brick](https://brickhub.dev/bricks/flutter_rust_bridge/)
to scaffold most of[^1] the code written here.

[^1]: Some setup steps are still required even with the brick, which we will go into more detail in the later sections.
      The brick is a work-in-progress.
