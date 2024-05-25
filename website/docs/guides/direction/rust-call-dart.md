# Rust calls Dart

Without this feature, the bridge would be a one-way road; but now we have two-way road ;)
In short, Rust can call arbitrary Dart functions (or "closures", or "callbacks").

## Simple example

Suppose we have a Rust function that accepts a function (closure):

```rust
pub fn rust_function(dart_callback: Fn(String) -> String) {
    dart_callback("Tom"); // Will get `Hello, Tom!`
}
```

Then we can provide a *Dart* closure:

```dart
rustFunction(dartCallback: (name) => 'Hello, $name!');
```

## Make it compile

To make it compile, we need a bit of boilerplate, and here is the real code.
No worries, they are just syntax noise and does not carry anything special.

```rust
pub async fn rust_function(dart_callback: impl Fn(String) -> DartFnFuture<String>) {
    dart_callback("Tom".to_owned()).await; // Will get `Hello, Tom!`
}
```

```dart
await rustFunction(dartCallback: (name) => 'Hello, $name!');
```

## More complicated

Features mentioned in other sections are supported here as well.
For example, you can:

* Use arbitrary non-encodable / non-transferable Dart objects as argument / return values of the Dart closure (via `DartOpaque`).
* Let Rust call Dart which calls Rust which calls Dart which calls Rust ;)
* ...
