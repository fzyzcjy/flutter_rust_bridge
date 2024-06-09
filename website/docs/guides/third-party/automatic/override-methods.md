# Override/add methods

In this page, we show how to do these in the example below:

* Change the function signature / method implementation / ... of something in a third-party crate
* Add arbitrary methods to existing structs/enums in third party crates

## (Optional) How it is done

How is the example below implemented? It is just a combination of two simple things:

* There are convenient Rust tools (e.g. [extend](https://crates.io/crates/extend), [easy-ext](https://crates.io/crates/easy-ext/)) to implement the "extension trait pattern".
* `flutter_rust_bridge` understands the [`frb_override_` prefix](../../miscellaneous/override-prefix)

## Example

### Override existing methods

Suppose the third party crate has code like:

```rust
pub struct S { ... }
impl S {
    pub fn greet(&self, name: &str) { ... }
}
```

Then we can override the `greet` function like:

```rust
use extend::ext; // or, for example, easy_ext's
use the_external_crate::path::to::S;

#[ext]
pub impl S {
    pub fn frb_override_greet(&self, age: i32, first_name: String, last_name: Vec<u8>) {
        // We can have arbitrary implementation.
        // Here, we demonstrate how to call the original implementation with modified arguments.
        self.greet(format!("{age}-{first_name}-{last_name:?}"))
    }
}
```

### Add new methods

It is very similar to the approach above, except that we do not need to prefix with `frb_override_`.
