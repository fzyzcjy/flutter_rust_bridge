# Methods

In addition to standard functions,
there is support for structs with methods. Both static methods, and non-static methods are supported.
No special syntax is needed, and just write normal `impl YourStruct { pub fn your_method() {} }`.

For methods in other crates, please refer to [this page](../third-party/manual/external-types) and the more general [feature](../third-party).

## Example

### Example 1: Methods in same crate

```rust
pub struct SumWith { pub x: u32 }

impl SumWith {
    pub fn sum(&self, y: u32) -> u32 { self.x + y }
    pub fn sum_static(x: u32, y: u32) -> u32 { x + y }
}
```

Becomes:

```dart
class SumWith {
  final int x;

  const SumWith({
    required this.x,
  });

  Future<int> sum({required int y, dynamic hint}) { ... }

  static Future<int> sumStatic({required int x, required int y, dynamic hint}) { ... }
}
```

Remark: If you are curious about `Future`, have a look at [this](../concurrency/async-dart).

### Example 2: Methods in external crates

The documentation was moved to [this page](../third-party/manual/external-types).
The new feature - automatically scanning a whole third party package - may also be helpful and is discussed [here](../third-party).
