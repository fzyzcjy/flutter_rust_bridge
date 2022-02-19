# Opaque pointers

Opaque pointers allow Dart to _own_ pointers to some data in memory. Rust
functions can either return opaque pointers to transfer ownership of the data,
or accept them as parameters to extract the pointer's contents.

`Opaque<T>` is valid for any T, with the only limitation being that T implements
the `DartSafe` marker trait, which itself requires that T implements:

- `Send` and `Sync`: Dart and Rust functions run in separate threads, therefore
  types returned from Rust must also be safe to send to and share with Dart.
- `UnwindSafe`: This library uses `catch_unwind` to convert Rust panics to
  proper Dart exceptions, so the return type must also be safe to unwind.
- `RefUnwindSafe`: Similar to above, but for shared references. This trait
  disqualifies all interior mutability types, such as Cell, RefCell, etc., and
  types that depend on UnsafeCell.

`DartSafe` is automatically implemented for types that implement all of these
traits.

This requirement has one consequence: trait objects e.g. `Box<dyn Trait>` must
also implement `DartSafe` to be compatible with Opaque. Here is a common way to
define opaque pointers to trait objects:

```rust,ignore
// first define a trait that exhibits the desired traits
pub trait MyTrait: DartSafe + Clone + .. {}
// auto implement the trait for qualifying types
impl<T: DartSafe + Clone + ..> MyTrait for T {}
// use the trait
pub struct MyWrapper(pub Opaque<dyn MyTrait>);
```
