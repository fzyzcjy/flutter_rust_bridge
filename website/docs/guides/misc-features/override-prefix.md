# `frb_override_*` prefix

If a method name starts with `frb_override_`,
then it will be automatically recognized as if it does not have that prefix,
and it gains privilege to override other existing methods (i.e. remove methods with same name).

This is helpful when wanting to override existing methods,
and also helpful when the name conflicts (such as in `#[ext]`).

We may extend this to other things as well in the future, such as struct names.

## Example

```rust
impl MyStruct {
    pub fn frb_override_hello(&self, a: i32) -> i32 {}
}
```

Then it is equivalent to `fn hello(...)` with privilege.
