# Tuples

Introduced in Dart 3, [records](https://dart.dev/language/records) provide the equivalent of Rust's tuples.
Tuples of up to 10 elements are supported, and more can be added by nesting tuples. Tuples can be returned,
received as parameters, and stored inside structs.

Tuples of one single element, i.e. `(T,)` is not supported yet, since that can be usually replaced by `T` itself or the newtype pattern.

```rust
pub fn my_coordinate() -> (f64, f64);
```

```dart
(double, double) myCoordinate();
final (lat, long) = myCoordinate();
```
