# Callables

Your Rust struct can be a callable object at the Dart side.
Please refer to the comments in the example below for more details.

## Example

```rust
pub struct A { ... }

impl A {
    pub fn call(&self, my_arg: String) -> String { ... }
}
```

Then, you can call it in Dart like:

```dart
// Obtain an object of type A by whatever approach
var a = A();
// Though the `a` is an *object*, it acts as if it is a *function*
a(myArg: 'hello');
```
