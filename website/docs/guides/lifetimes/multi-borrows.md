# Multi borrows

Suppose we have `fn f(foo: &Foo) -> &Bar { .. }`.
Then, before the `Bar` object is disposed (or is garbage collected),
the `foo` can only be used immutably and not mutably.
More specifically,

```dart
var foo = Foo();
var bar = f(foo);
// functionThatMutablyBorrowFoo(foo); // <-- cannot do it here
bar.dispose();
functionThatMutablyBorrowFoo(foo); // <-- can do it here
```

This is a limitation somehow caused by the combination of Rust and Dart:
In Rust, we have the strict limitation that, if a type is borrowed immutably,
then we can never borrow it mutably, *unless* the former borrow is removed.
Therefore, in the example above, when `bar` object is alive, `foo` is borrowed,
and we cannot have another mutable borrow.

If this makes your scenario much more complicated,
feel free to open an issue to discuss your scenario,
and we can discuss how to improve it.
For example, we may introduce something like a "scope" where references out of scope will be disposed.

In some scenarios, due to this limitation, the alternatives introduced in the next page may be appealing.
