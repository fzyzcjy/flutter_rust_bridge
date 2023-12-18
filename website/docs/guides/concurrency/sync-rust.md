# Synchronous Rust (thread pool)

You can write normal (non-`async`) Rust functions (`fn f() {}`).
Multiple Rust functions can be running at the same time,
and they will be running concurrently.
This is because by default we use a thread pool to execute the Rust functions.
However, you can fully customize this behavior (and even throw away the thread pool).

## Example

Consider the following Rust code:

```rust
pub fn compute() {
  thread::sleep(Duration::from_millis(1000));
}
```

And the following Dart code using it:

```dart
var a = compute();
var b = compute();
var c = compute();
await Future.wait([a, b, c]); // You may need to learn `Future` and `async` in Dart to understand this
```

Then it will take 1 second instead of 3 seconds to complete the code, because multiple `compute` can run concurrently.

## Customization

By [providing your custom Rust `Handler`](../custom/rust),
you can customize configurations of the thread pool (worker pool).

If you want to change the number of threads of the pool,
create your own instance of the struct `SimpleThreadPool`,
and provide it to the `Handler`.
(TODO: We should add an API to construct such custom-number-of-threads pools more easily.
Create an issue if you want to know and this is still not updated.)

Alternatively, you can even use other crates to provide a thread pool,
by simply implementing the trait `BaseThreadPool` (which has only one method - `execute`).
Your implementation can be anything - not even necessarily be a real thread pool.

## With synchronous Dart mode

If you are using synchronous Dart mode,
alternatively, the Rust code will be executed on the main thread
instead of the thread pool mentioned here.
