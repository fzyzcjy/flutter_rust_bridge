# Methods

In addition to standard functions,
there is support for structs with methods. Both static methods, and non-static methods are supported.
No special syntax is needed, and just write normal `impl YourStruct { pub fn your_method() {} }`.

## Methods in external crates

For methods that are not defined in the `rust_input` folders in the current crate,
the `#[frb(external)]` syntax (see example below) is needed to make flutter_rust_bridge aware of the methods.

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

Suppose we have these in external crates:

```rust
pub struct MyExternalStruct {
    ...
}

impl MyExternalStruct {
    pub fn simple_external_method(&self) -> String {
        // ... some long implementations ...
    }
}
```

Then, we only need to repeat the function signatures in our main crate as follows:

```rust
#[frb(external)]
impl MyExternalStruct {
    pub fn simple_external_method(&self) -> String {}
}
```

Remark: Just leave the function body empty (i.e. `{}`), no need to put anything there.

This feature is compatible with the [mirroring](../types/translatable/external/diff-crate) feature as well.
