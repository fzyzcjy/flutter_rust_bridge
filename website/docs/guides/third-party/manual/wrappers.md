# Wrappers

This is not a `flutter_rust_bridge` feature,
but an approach that can be useful here,
and is discussed for completeness.

## Example

Suppose in third party crate there is a type:

```rust
pub struct Foo { .. }

impl Foo {
    pub fn method_one(..) -> .. { .. }
    pub fn method_two(..) -> .. { .. }
    ...
}
```

And suppose it has some special fields making it not `Sync`, thus cannot directly be used.
For simplicity, let us wrap it using a `Mutex`:

```rust
pub struct Foo(Mutex<third_party_crate::Foo>);
```

Then, how can we expose the methods? We can surely write down each and every one manually:

```rust
// Naive approach, do NOT use
impl Foo {
    pub fn method_one(..) -> .. {
        self.0.lock().unwrap().method_one(..)
    }
    pub fn method_two(..) -> .. {
        self.0.lock().unwrap().method_two(..)
    }
    ...
}
```

But that's tedious. We can do it faster by using [delegate-attr](https://crates.io/crates/delegate-attr) or something liek that.
Essentially, it does nothing but helping us generate those method bodies.
Then the code looks like:

```rust
#[delegate(self.0.lock().unwrap())] // <-- just add this, and leave the method body empty
impl Foo {
    pub fn method_one(..) -> .. {}
    pub fn method_two(..) -> .. {}
    ...
}
```
