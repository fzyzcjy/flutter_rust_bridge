# `RustAutoOpaque` class

Sometimes you may want to directly work with the `RustAutoOpaque<T>` class.
The main API is quite simple:

```rust
fn example() {
    let opaque = RustAutoOpaqueNom::new(42);
    *opaque.try_write().unwrap() = 100;
    println!("{}", opaque.try_read().unwrap());
}
```

There are several variants for reading and writing:

* `try_read`, `try_write`
* `read`, `write`
* `blocking_read`, `blocking_write`
