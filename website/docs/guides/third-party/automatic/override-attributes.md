# Override attributes

If the implementations of an external component (function/method/struct/enum/etc) is good,
and we only want to add some *attributes* to it,
then we only need to write down component name and new attributes inside our first-party package like below.

## Examples

### Example: Making methods synchronous getters

Suppose we are interested in the `web_audio_api::AudioParam::value` method in third-party crate:

```rust
// This is code in third-party crate, we cannot modify it
pub struct AudioParam {
    ...
}

impl AudioParam {
    pub fn value(&self) -> f32 {
        ...
    }
}
```

Then, we can write down the following lines to make that method a synchronous getter:

```rust
// src/third_party/web_audio_api/mod.rs
#[frb(external)]
impl AudioParam {
    #[frb(sync, getter)] // <-- This attribute will be auto merged to third-party code
    pub fn value() {}
}
```

Remarks:

* No need to repeat function signatures (such as `&self`, `f32`, etc) - the function name itself is sufficient.
* Please put the code inside `src/third_party/{third-party-crate-name}/{path-to-the-module}`. For example, for `hello::a::b::C` we need to put in `src/third_party/hello/a/b.rs`

### Example: Ignoring a function

It is quite similar to the example above. Indeed, just replace the attributes above with `#[frb(ignore)]`.
