# Map and Set

The `HashMap<K, V>` as well as `HashSet<T>` are supported as well.

## Example

Suppose the Rust code is:

```rust
pub fn f(a: HashMap<String, Vec<u8>>) {}
pub fn g(a: HashSet<String>) {}
```

Then it will be `Map<String, Uint8List>` and `Set<String>` on the Dart side.
