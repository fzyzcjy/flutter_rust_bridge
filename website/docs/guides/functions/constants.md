# Constants

Rust constants will be automatically translated as well.
Same as functions, it needs to be `pub` to be included.

## Example

```rust
pub const CONST_INT: i32 = 42;
```

It will provide the following getter automatically:

```dart
int get constIntTwinNormal => ...;
```
