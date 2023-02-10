# Handler

By default, the `DefaultHandler` is used for handling function calls. You can implement your own `Handler` with other custom behaviors you want. In order to do this, create a module variable named `FLUTTER_RUST_BRIDGE_HANDLER` in `api.rs`(probably using `lazy_static`) of your project. You may not need to create a brand new struct implementing `Handler`, but instead, use the `SimpleHandler` and customize its generic arguments such as its `Executor`.

## Examples

### Example: Report errors to your backend in addition to telling Dart

```rust,noplayground
pub struct MyErrorHandler(ReportDartErrorHandler);

impl ErrorHandler for MyErrorHandler {
    fn handle_error(&self, port: i64, error: handler::Error) {
        send_error_to_your_backend(&error);
        self.0.handle_error(port, error)
    }

    ...
}
```

### Example: Log when execution starts and ends

```rust,noplayground
pub struct MyExecutor(ThreadPoolExecutor<MyErrorHandler>);

impl Executor for MyExecutor {
    fn execute<TaskFn, TaskRet>(&self, wrap_info: WrapInfo, task: TaskFn) {
        let debug_name_string = wrap_info.debug_name.to_string();
        self.thread_pool_executor
            .execute(wrap_info, move |task_callback| {
                Self::log_around(&debug_name_string, move || task(task_callback))
            })
    }
}

impl MyExecutor {
    fn log_around<F, R>(debug_name: &str, f: F) -> R where F: FnOnce() -> R {
        let start = Instant::now();
        debug!("(Rust) execute [{}] start", debug_name);
        let ret = f();
        debug!("(Rust) execute [{}] end delta_time={}ms", debug_name, start.elapsed().as_millis());
        ret
    }
}
```

### Example: Use a simple handler

```rust,noplayground
// api.rs

use flutter_rust_bridge::handler::ReportDartErrorHandler;
use flutter_rust_bridge::handler::SimpleHandler;
use flutter_rust_bridge::handler::ThreadPoolExecutor;
use lazy_static::lazy_static;

lazy_static! {
    static ref FLUTTER_RUST_BRIDGE_HANDLER:
    SimpleHandler<ThreadPoolExecutor<ReportDartErrorHandler>, ReportDartErrorHandler> =
        SimpleHandler::new(
            ThreadPoolExecutor::new(ReportDartErrorHandler),
            ReportDartErrorHandler {}
        );
}
```
