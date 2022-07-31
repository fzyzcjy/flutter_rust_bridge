# Result

There is support for Rust's `std::Result`, where you can return either a value or an error.

## `anyhow`

You can use `anyhow::Result` as a return value, and backtraces will be captured automatically.

## Custom errors

Errors with custom fields are also supported, and you can even pass a backtrace.

### Example 1: Without backtrace

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
}
```

### Example 2: With backtrace

Errors with custom fields are also supported, and you can even pass a backtrace:

```rust
pub enum CustomStructError {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}
```

As for how to fill it in or use it, you can refer to `thiserror` crate for some hints.
