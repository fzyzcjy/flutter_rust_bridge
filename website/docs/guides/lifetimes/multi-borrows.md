# Multi borrows

In this page, we discuss a limitation of types with lifetimes because of combination of Rust and Dart.

## A pure Rust example

Let's get started with a Rust example (full code [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=13d436ffe4da5d746837941e3f5b72b5)):

```rust
fn f(foo: &Foo) -> &Bar { .. }
fn function_that_mutably_borrow_foo(foo: &mut Foo) { .. }

let mut foo = Foo();
let bar = f(&foo);

// function_that_mutably_borrow_foo(&mut foo); // <-- cannot do it here, since bar is still borrowed

println!("{}", bar);
drop(bar);

function_that_mutably_borrow_foo(&mut foo); // <-- can do it here
```

Rust compiler does not allow us to call the `function_that_mutably_borrow_foo` at the first place,
because `bar` immutably borrows `foo` and is still alive.
Recall that we are forbidden from having both immutable borrows and mutable borrows on the same object.
(Uncomment the line in Rust playground to see it in action.)

## Translate to Dart

The equivalent code written in Dart is as follows.

```dart
var foo = Foo();
var bar = f(foo);
// functionThatMutablyBorrowFoo(foo); // <-- cannot do it here
bar.dispose();
functionThatMutablyBorrowFoo(foo); // <-- can do it here
```

As can be expected, it has the same issue,
except that the form of issue will be waiting at runtime instead of errors at compile time.

In addition,
though we usually do not need to manually call `dispose` (because GC will do it for us),
here we need to do it manually.
This is because we want `bar` not to borrow `foo` after that one,
and there is no (at least yet) deep static analysis on Dart to automatically do so.

Therefore, the takeaway is:
If we want to borrow `foo` mutably,
remember to `dispose` the `bar` object before that.

If this makes your scenario much more complicated,
feel free to [open an issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues) to discuss your scenario,
and we can discuss how to improve it.
For example, we may introduce something like a "scope" where references out of scope will be disposed.

In some scenarios, the alternatives introduced in the next page may be appealing because of this.
