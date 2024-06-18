# External methods

:::info
Third-party packages can be converted automatically; only use the feature in this page when the automation is unwanted.
:::

For methods that are not defined in the `rust_input` folders in the current crate,
the `#[frb(external)]` syntax (see example below) is needed to make flutter_rust_bridge aware of the methods.

## Example

Suppose we have these in external crates:

```rust
pub struct MyExternalStruct {
    ...
}

impl MyExternalStruct {
    pub fn simple_external_method(&self) -> String {
        // ... some long implementations ...
    }
}
```

Then, we only need to repeat the function signatures in our main crate as follows:

```rust
#[frb(external)]
impl MyExternalStruct {
    pub fn simple_external_method(&self) -> String {}
}
```

Remark: Just leave the function body empty (i.e. `{}`), no need to put anything there.

This feature is compatible with the [mirroring](../../types/translatable/external/diff-crate) feature as well.
