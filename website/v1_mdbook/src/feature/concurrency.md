# Concurrency

Multiple Rust functions can be running at the same time, and they will be running concurrently. This is because by default we use a thread pool to execute the Rust functions. However, you can fully customize this behavior (and even throw away the thread pool).

## Example

Consider the following Rust code:

```rust,noplayground
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