# Overview

Surely, functions are supported. For example,

```rust
fn f(a: String, b: Vec<String>) -> MyStruct { ... }
```

can be called from Dart, without manual intervention:

```dart
print(f(a: 'Hello', b: ['Tom']));
```

In the following sections, more advanced functionalities related to functions will be introduced,
such as methods, properties, constructors, etc.
