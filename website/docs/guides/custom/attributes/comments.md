# Specify attributes using comments

The behavior can often be customized via attributes as well.
For example, to make a function synchronous on Dart side, we can put `#[frb(sync)]` on that function.

Not only can we write down `#[frb(something)]`,
but we can also write comments `/// flutter_rust_bridge:something` to do the same thing.

The latter is especially useful when the former cannot be used,
such as when the target is a `mod`, or when the crate does not have dependency on `flutter_rust_bridge`.

Most of the time, the latter is equivalent to the former;
but for things like `#[frb(external)]`, which has to act as a macro to generate some code,
the latter cannot be used.
But this can be easily spotted since it will have compile-time messages.

## Example

```rust
/// flutter_rust_bridge:ignore
pub fn f() {}
```

