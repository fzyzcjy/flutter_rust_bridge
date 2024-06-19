# Proxy

When annotating a method with `#[frb(proxy)]`,
the method will be allowed to return a reference type,
and the behavior will be explained below.

Typically, this can be used to expose struct or enum fields.

## Example

### Scenario

Consider the following code:

```rust
#[frb(opaque)]
pub struct BiquadFilterNode {
    frequency: AudioParam,
}

impl BiquadFilterNode {
    pub fn frequency(&self) -> &AudioParam {
        &self.frequency
    }
}

pub struct AudioParam { ... }

impl AudioParam {
    pub fn my_method_one(&self, value: f32) { ... }
    pub fn my_method_two(&self, value: f32) { ... }
}
```

`flutter_rust_bridge` will not be able to generate code for it, since the return type being a reference type
is not supported yet (and if implemented, may have problems such as "borrowing for too long").
However, if we add `#[frb(proxy)]` to the `fn`, then it will work well.

### Remark: Alternative solutions

As is mentioned in [this page](../lifetimes/alternatives), one alternative solution is to use `clone`:

```rust
pub fn get_my_sub_struct(&self) -> MySubStruct {
    self.frequency.clone()
}
```

Another solution is that, we can also utilize `Arc` or `RustAutoOpaque` (which is essentially an `Arc` with something else):

```rust
frequency: RustAutoOpaque<AudioParam>,
```

### (Optional) Under the hood

Shortly speaking, 
the generated code has similar idea to the code below, but the exact details is better.
I will elaborate more if you are interested in it.

```rust
impl BiquadFilterNode {
    pub fn frequency_my_method_one(&self, value: f32) {
        self.frequency.my_method_one(value)
    }

    pub fn frequency_my_method_two(&self, value: f32) {
        self.frequency.my_method_two(value)
    }
}
```
