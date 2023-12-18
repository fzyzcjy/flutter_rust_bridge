# Async (Tokio)

On non-web platforms, `tokio` is widely used for asynchronous Rust.
On web, [it is not](https://github.com/tokio-rs/tokio/issues/6178) supported super well.
Thus, we provide several functions that works on both non-web (via Tokio) and web:

* `spawn`
* `spawn_local`
* `spawn_blocking_with` (just like Tokio's `spawn_blocking`)

Please refer to the corresponding Tokio documentation and tutorials for their semantics.
