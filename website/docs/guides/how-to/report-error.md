# Report errors

To configure to report errors to your backend, in addition to telling Dart, we can simply use a [custom Handler](../func/handler):

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
