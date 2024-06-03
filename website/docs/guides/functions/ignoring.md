# Ignoring functions

This page explains how to let a function not to be translated by flutter_rust_bridge.

## Automatic ignore

All functions that are not public (`pub`) will be ignored by default.
This include private functions, functions with `pub(crate)`, `pub(super)`, etc.

## Explicit ignore

If it is public but still needs to be ignored,
you can use the attribute `#[frb(ignore)]` to ignore it.
For example:

```rust
#[frb(ignore)]
pub fn ignored_in_frb() {
    println!("you should not see ignoredInFrb in dart side.");
}
```

This feature also supports methods. For example:

```rust
struct RandomStruct;

impl RandomStruct {
    #[frb(ignore)]
    pub fn ignored_in_frb() {
        println!("you should not see ignoredInFrb within RandomStruct in dart side.");
    }
}
```

Currently `#[frb(ignore)]` doesn't support enums and structs yet - feel free to open an issue if your scenario needs it.
