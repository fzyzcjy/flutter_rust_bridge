# Return Types

The return type can be either `anyhow::Result<YourType>`, or `YourType` directly.

## Example

```rust,noplayground
pub fn f(a: i32, b: i32) -> i32 { a + b }

pub fn g(a: i32, b: i32) -> anyhow::Result<i32> { Ok(a + b) }
```

