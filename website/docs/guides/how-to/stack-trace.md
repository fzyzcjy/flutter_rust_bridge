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

## Other solutions

There are many products to help you capture errors and provide a central panel to look at them.
[This official Flutter page](https://docs.flutter.dev/cookbook/maintenance/error-reporting)
lists some of them: Bugsnag, Datadog, Firebase Crashlytics, Rollbar, or Sentry.
This will be especially useful for your app in production.

## Behavior on Windows

It seems that Rust+Windows(+Flutter) does not handle stack traces well,
which is not a bug of flutter_rust_bridge, but something on upstream.

The current discussions and workarounds are in https://github.com/fzyzcjy/flutter_rust_bridge/issues/2200.
