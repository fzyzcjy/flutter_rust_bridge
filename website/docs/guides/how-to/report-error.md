# Report errors

## At Dart side

Just follow Flutter's guide, or your error backend's guide, to setup the error handling.

It will catch all pure Dart errors, as well as all Rust errors that are propagated to Dart.

## Ensure Rust stack traces

If you are not seeing Rust stack traces, please refer to [this page](stack-trace) to setup.

## (Optional) At Rust side

:::info
I personally do this because my Rust code has symbols stripped.
Thus, to see the Rust stack traces, I have to integrate the error reporting SDK at this layer,
to let it capture enough information for symbolication.

For normal users, try to do the Dart side setup first, and only do this if that does not give you enough information.
:::

We can simply use a [custom Handler](../custom/rust):

```rust
pub struct MyErrorHandler(ReportDartErrorHandler);

impl ErrorHandler for MyErrorHandler {
    fn handle_error(&self, port: i64, error: handler::Error) {
        send_error_to_your_backend(&error);
        self.0.handle_error(port, error)
    }

    ...
}
```
