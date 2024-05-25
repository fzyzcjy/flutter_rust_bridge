# `RustAutoOpaque` struct

Sometimes you may want to directly work with the `RustAutoOpaque<T>` struct.
It is currently implemented as (roughly) an `Arc<RwLock<T>>`.
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

which mimics [tokio's RwLock](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html) semantics.

There needs to be a lock, because the object can be used by multiple Rust threads concurrently.
