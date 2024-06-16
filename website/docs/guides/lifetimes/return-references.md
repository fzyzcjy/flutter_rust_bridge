# Return references

Suppose we have the following function:

```rust
fn f<'a>(foo: &'a Foo) -> &'a Bar { .. }
```

The return type `&Bar` is not *yet* supported (but should be there in the future).
However, we can workaround it as follows, by creating a very simple wrapper struct:

```rust
pub struct BarReference<'a>(&'a Bar);

fn f<'a>(foo: &'a Foo) -> BarReference<'a> { .. }
```

Then, the scenario becomes what has been discussed in the last page, and flutter_rust_bridge can handle it.
