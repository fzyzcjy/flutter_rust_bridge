# Return Types

The return type can be either `anyhow::Result<YourType>`, or `YourType` directly. For exceptions (errors), please refer to [exceptions section](lang_exceptions.md) as well.

## Example

```rust,noplayground
pub fn f(a: i32, b: i32) -> i32 { a + b }

pub fn g(a: i32, b: i32) -> anyhow::Result<i32> { Ok(a + b) }
```

