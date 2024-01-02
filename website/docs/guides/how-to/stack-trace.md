# Stack Traces

:::tip
If using the template by `flutter_rust_bridge_codegen create/integrate`, this is already configured by default,
via the auto-generated call to `flutter_rust_bridge::setup_default_user_utils()`.
:::

To pass Rust stack traces to flutter, you need to set `RUST_BACKTRACE` in the running application. For that simply add `env::set_var("RUST_BACKTRACE", "1");` before initialising the bridge.

Note: The `--dart-define` will not work, you **must** use `env::set_var`, because the former does not set the "environment variable" in the common sense, but instead a special thing only visible to Dart.