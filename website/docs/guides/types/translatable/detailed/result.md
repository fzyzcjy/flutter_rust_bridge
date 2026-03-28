# Result (Oxidized)

By default, when a Rust function returns `Result<T, E>`, any error is converted to a Dart exception. However, you can opt-in to receive Rust-style `Result` types in Dart using the [oxidized](https://pub.dev/packages/oxidized) package.

## Setup

Add the oxidized package to your Dart/Flutter dependencies in `pubspec.yaml`:

```yaml
dependencies:
  oxidized: ^6.2.0
```

## Configuration

There are three ways to enable oxidized Result types:

### 1. Auto-detection (default)

If the `oxidized` package is detected in your project's dependencies, fallible functions will automatically return `Result<T, E>` instead of throwing exceptions.

### 2. Project-wide configuration

Add to your `flutter_rust_bridge.yaml`:

```yaml
use_oxidized: true
```

### 3. Per-function annotation

Use the `#[frb(oxidized)]` attribute on individual functions:

```rust
#[frb(oxidized)]
pub fn my_fallible_function() -> Result<String, MyError> {
    // ...
}
```

## Example

### Rust

```rust
pub struct MyError {
    pub message: String,
}

#[frb(oxidized)]
pub fn divide(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError { message: "division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}
```

### Dart

```dart
final result = await divide(a: 10, b: 2);

if (result.isOk()) {
  print('Result: ${result.unwrap()}');
} else {
  print('Error: ${result.unwrapErr().message}');
}

// Or use pattern matching
result.when(
  ok: (value) => print('Result: $value'),
  err: (error) => print('Error: ${error.message}'),
);
```

## Type Mapping

| :crab: Rust          | :dart: Dart                                                      |
| -------------------- | ---------------------------------------------------------------- |
| `Result<T, E>`       | `Result<T, E>` from [oxidized](https://pub.dev/packages/oxidized) |
| `anyhow::Result<T>`  | `Result<T, AnyhowException>`                                     |

:warning: Please note that you need to add the [oxidized](https://pub.dev/packages/oxidized/install) package to your Dart/Flutter dependencies in `pubspec.yaml`.
