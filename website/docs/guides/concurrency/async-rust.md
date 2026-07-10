# Asynchronous Rust

The async Rust functions, `async fn f() {}`, are supported as well.
Just write normal Rust code, and flutter_rust_bridge code generator will recognize it is async
and generate proper bindings.
Under the hood, async runtime is utilized to run those functions.

On native platforms, the default runtime moves async tasks between worker threads, so their futures must implement `Send`. When a dependency exposes a non-`Send` future, annotate the exported async function with `#[frb(local)]`:

```rust
#[frb(local)]
async fn f() { ... }
```

This keeps the future on the FFI calling thread and still exposes a Dart `Future`. It runs within a Tokio `LocalSet`, so `spawn_local` is available. It is intended for thread-affine dependencies and can block that calling thread until the Rust future completes, so use it only when that trade-off is acceptable. In particular, a local future must not wait for work on the Dart isolate that made the call, including a Dart callback, because that isolate cannot process the work until the FFI call returns.

As for when to use asynchronous vs synchronous Rust,
there are already many articles on the Internet,
for example, by searching "Rust async vs thread pool", "why async Rust", etc.
In short, for example,
when you want Rust to read/write files, do network requests, etc,
it is better to use async.
On the other hand,
if your computation is CPU-heavy,
the thread pool may fit better.

## Example

Well, just write anything you like, so here is a bare minimal example:

```rust
async fn f() { ... }
```

With Dart:

```dart
await f();
```

## Customization

Similar to how you customize thread pools for [synchronous Rust](sync-rust.md),
we can also provide arbitrary async-runtime implementations.
This can be done similarly by creating your custom handler instance with custom async runtime.

For example, you may want to change the number of OS threads that Tokio creates.
Or, it is also easy to plug in whatever async runtime that you like,
by implementing the `BaseAsyncRuntime` trait. Existing custom runtimes only need `spawn`; the default `block_on_local` implementation reports that `#[frb(local)]` is unavailable. Implement it when custom handlers need to support local async APIs.
