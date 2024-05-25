# Renaming functions

To customize the name on Dart side to be different from the name on Rust side, the `#[frb(name = ...)]` can be used.

## Example

For example,

```rust
#[frb(name = "g")]
pub fn f() {}
```

will generate function named `g` on Dart side.
