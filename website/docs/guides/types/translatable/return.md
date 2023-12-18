# Return Types & Exceptions

Suppose your function wants to return a `String`, then all these return types are supported:

1. Direct return type (`fn f() -> String`)
2. Anyhow error (`fn f() -> anyhow::Result<String>`): The anyhow error will be automatically converted to a Dart exception.
3. Arbitrary custom error type (`fn f() -> Result<String, YourErrorType>`): The `YourErrorType` will be automatically converted to a Dart exception.

In addition, Rust has `panic` in addition to Result error, thus:

* When Rust panic occurs, a `PanicException` will be thrown in Dart.

If you want to see stack traces (backtraces), [this doc page](../../how-to/stack-trace) discusses how to configure it.

## Example

### Example 1: Direct Result

```rust
pub fn f(a: i32, b: i32) -> i32 { a + b }
```

### Example 2: Anyhow Result

For example, the following code, when called by Dart code, will throw Dart exceptions.

```rust
pub fn f() -> anyhow::Result<i32> { bail!("oops I failed") }
```

### Example 3: Panic

All functions below, when called, will throw Dart exceptions at the Dart side due to the `panic`.

```rust
pub fn g1() -> i32 { panic!("oops I failed") }
pub fn g2() -> anyhow::Result<String> { panic!("oops I failed") }
pub fn g3() -> Result<Vec<u8>, CustomError> { panic!("oops I failed") }
```

### Example 4: Custom Error Without backtrace

```rust
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

### Example 5: Custom Error With backtrace

Errors with custom fields are also supported, and you can even pass a backtrace:

```rust
pub enum CustomStructError {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}
```

As for how to fill it in or use it, you can refer to `thiserror` crate for some hints.
