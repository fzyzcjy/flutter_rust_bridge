# Customize handler

By default, a `FLUTTER_RUST_BRIDGE_HANDLER` instance is automatically generated
(by the auto-generated `flutter_rust_bridge::frb_generated_default_handler!()` macro),
and you can see it at `frb_generated.rs`.
However, you can provide your own `FLUTTER_RUST_BRIDGE_HANDLER` instance.
If your instance is detected, the generator will not generate one, but will use your version.

As for how to write a custom handler, often copy-paste-modify the code
in `flutter_rust_bridge::frb_generated_default_handler!()` is a good idea.

The handler is the central entrypoint to handle calls between Rust and Dart,
therefore please visit the API of the `Handler` trait for more details.

Some typical scenarios are:

* [Inspection](../../how-to/inspect)
* [Report errors](../../how-to/report-error)
* Customizing executors, thread pools, async runtimes, ...

The API of handler may have breaking change across minor version bumps,
which is unlike most parts of flutter_rust_bridge which follows semantics versioning.
However, usually it can be easily migrated by adding the corresponding parameters in your code.

## States in handler

If you are only using the provided handler and executor (with your own arguments),
you can ignore this section. Only read it when you want to create your brand new handler.

It is usually a good idea to avoid coupling the handler with the specific Dart side isolate.
For example, suppose whenever we call `RustLib.init()` on Dart side,
we create a Dart isolate port and let the Rust side save and use it.
Then, during `flutter test`s, multiple Dart test files will be executed concurrently,
but (from my experiments) they all share one single Rust side.
Then, the one Rust side will be initialized multiple times with different Dart ports,
and it causes confusion and bugs.
Similar things may happen if you are using multiple isolates in your Dart program,
or when you hot-restart the Dart side.

