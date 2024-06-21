# Ignoring things

The `#[frb(ignore)]` (or equivalently, `/// flutter_rust_bridge:ignore`) can be put on things to ignore them.

To ignore a `mod` (module), we can only use the latter syntax because Rust grammar does not support the former yet.

To see extra approaches to ignore functions, please refer to [this page](../functions/ignoring).

## Example

```rust
#[frb(ignore)]
pub struct ThisStructWillBeIgnore;

#[frb(ignore)]
pub fn this_function_will_be_ignored() {}

/// flutter_rust_bridge:ignore
mod this_submodule_will_be_ignored;
```
