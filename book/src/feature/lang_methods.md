# Methods

There is support for structs with methods

## Example

```rust,noplayground
pub struct SumWith {
    pub x: u32,
}

impl SumWith {
    pub fn sum(&self, y: u32) -> u32 {
        self.x + y 
    }
}
```

Becomes:

```Dart

class SumWith {
  final FlutterRustBridgeExampleSingleBlockTest bridge;
  final int x;

  SumWith({
    required this.bridge,
    required this.x,
  });
  Future<int> sum({required int y, dynamic hint}) => ..
}
```

Remark: If you are curious about `Future`, have a look at [this](async_dart.md).