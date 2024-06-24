# Override attributes

If the implementations of an external component (function/method/struct/enum/etc) is good,
and we only want to add some *attributes* to it,
then we only need to write down component name and new attributes inside our first-party package like below.

The rule is that:

* Put the code inside `src/third_party/{third-party-crate-name}/{path-to-the-module}`.
* Only write down function name, and no need to specify function arguments and return types.

## Examples

### Example 1: Struct methods

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

### Example 2: Trait definition methods

Support `SomeTrait` is a third party trait, then again we can do something like above.

```rust
#[frb(external)]
impl SomeTrait {
    #[frb(sync)]
    pub fn method() {}
}
```

### Example 3: Trait implementation methods

Suppose we have third party `SomeTrait` and `SomeStruct` implementing it.
Then, to override the implementation method, we can do:

```rust
#[frb(external)]
impl MyTrait for MyStruct {
    #[frb(ignore)]
    fn method(a: i32, b: String) -> Vec<f64> {}
}
```

Note that, currently, we have to specify the function parameters (but still no need to have a body).
This syntax may be simplified (i.e. no need to specify) in the future,
and may not follow semantics versioning.

## Remark on `pub use`

`flutter_rust_bridge` understands syntax like `pub use something::*` and `pub use another::Thing`.
Therefore, if a struct in third party code is defined non-publicly but then re-exported as public using such `pub use` grammar,
`flutter_rust_bridge` will consider that type to be in the latter module.
