# Override attributes

If the implementations of an external component (function/method/struct/enum/etc) is good,
and we only want to add some *attributes* to it,
then we only need to write down component name and new attributes inside our first-party package like below.

The rule is that:

* Put the code inside `src/third_party/{third-party-crate-name}/{path-to-the-module}`.
* Only write down function name, and no need to specify function arguments and return types.

## Examples

Suppose we are interested in the `some_crate::hello::world::SomeStruct::method` method in third-party crate:

```rust
// This is code in third-party crate, we cannot modify it
pub struct SomeStruct {
    ...
}

impl SomeStruct {
    pub fn method(&self, a: String, b: Vec<i32>) -> f32 {
        ...
    }
}
```

Then, we can write down the following lines to make that method has synchronous Dart code:

```rust
// src/third_party/some_crate/hello/world/mod.rs
#[frb(external)]
impl SomeStruct {
    #[frb(sync)] // <-- This attribute will be auto merged to third-party code
    pub fn method() {}
}
```
