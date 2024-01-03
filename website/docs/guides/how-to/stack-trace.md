# Stack Traces

:::tip
This is standard Rust behavior, and is unrelated to flutter_rust_bridge.
However, we do provide defaults to be "battery included".

If using the template by `flutter_rust_bridge_codegen create/integrate`, this is already configured by default,
via the auto-generated call to `flutter_rust_bridge::setup_default_user_utils()`.
:::

## Errors (`Result`)

To pass Rust stack traces to flutter, you need to set `RUST_BACKTRACE` in the running application. For that simply add `env::set_var("RUST_BACKTRACE", "1");` before initialising the bridge.

Note: The `--dart-define` will not work, you **must** use `env::set_var`, because the former does not set the "environment variable" in the common sense, but instead a special thing only visible to Dart.

## Panics

The standard Rust does not provide stack traces when catching a panic.
If you want to know the stack traces, you can use approaches like
https://stackoverflow.com/questions/69593235/how-to-get-panic-information-i-e-stack-trace-with-catch-unwind.
The builtin utils do use it.
