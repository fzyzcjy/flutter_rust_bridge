# Result / Exceptions

1. For Result/Error, the`anyhow::Result`/`anyhow::Error` is supported. It will be automatically converted to a Dart Exception.
2. For `panic`s, it will also be automatically captured and converted to Dart exceptions.
3. For error hierarchy, or arbitrary error types, it is also supported. For example, you can create your own `CustomError` (such as using `thiserror`), and it will automatically be converted to a new Dart class.

If you want to see stack traces (backtraces), [this doc page](stack_trace.md) discusses how to configure it.

## Example

### Example 1: Anyhow Result

For example, the following code, when called by Dart code, will throw Dart exceptions.

```rust,noplayground
pub fn f() -> anyhow::Result<i32> { bail!("oops I failed") }
```

### Example 2: Panic

All functions below, when called, will throw Dart exceptions at the Dart side due to the `panic`.

```rust,noplayground
pub fn g1() -> i32 { panic!("oops I failed") }
pub fn g2() -> anyhow::Result<String> { panic!("oops I failed") }
pub fn g3() -> Result<Vec<u8>, CustomError> { panic!("oops I failed") }
```

### Example 3: Custom Error Without backtrace

```rust,noplayground
pub enum CustomError {
    Error0(String),
    Error1(u32),
}

pub fn return_err_custom_error() -> Result<u32, CustomError> {
    Err(CustomError::Error1(3))
}
```

Becomes something that can be used like this:

```Dart
try {
    final r = await api.returnErrCustomError();
    print("received $r");
} catch (e) {
    print('dart catch e: $e');
    expect(e, isA<CustomError>());
}
```

### Example 4: Custom Error With backtrace

Errors with custom fields are also supported, and you can even pass a backtrace:

```rust,noplayground
pub enum CustomStructError {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}
```

As for how to fill it in or use it, you can refer to `thiserror` crate for some hints.
