# Ignoring Functions
Sometimes you may want public functions not to be translated by frb.
You may use the attribute `#[frb(ignore)]` to ignore it.

```rust
#[frb(ignore)]
pub fn ignored_in_frb() -> {
  println!("you should not see ignoredInFrb in dart side.");
}
```

Also supports methods.
```rust
struct RandomStruct;
impl RandomStruct {
  pub fn ignored_in_frb() -> {
    println!("you should not see ignoredInFrb within RandomStruct in dart side.");
  }
}
```

Currently `#[frb(ignore)]` doesn't support enums and structs, keep that in mind.
