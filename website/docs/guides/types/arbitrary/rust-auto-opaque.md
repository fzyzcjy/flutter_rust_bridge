# Automatic arbitrary Rust type

This feature, sometimes called `RustAutoOpaque` throughout the documentation,
allows arbitrary Rust type to be used without manual intervention,
by representing arbitrary Rust object as (smart) pointers in Dart.

Different from non-opaque types, opaque types are not copied/moved/reconstructed at all.
For example, if you pass around `RwLock<Mutex<ArbitraryData>` in arguments and return values,
you will get the exact *same* `RwLock<ArbitraryData>` object.

## Example

Suppose you have a type that is not encodable:

```rust
pub struct MyNonEncodableType {
    // e.g., a temporary directory, a file descriptor, a native resource, a lock, a channel, ...
    sample_non_encodable_field: tempdir::TempDir,
}
```

Then you can have Rust functions and methods on it.
Most, if not all, features of flutter_rust_bridge are supported,
and here are a few examples:

```rust
pub fn create() -> MyNonEncodableType { ... }

pub fn consume(obj: MyNonEncodableType) { ... }

pub fn borrow(obj: &MyNonEncodableType) { ... }

pub fn mutable_borrow(obj: &mut MyNonEncodableType) { ... }

impl MyNonEncodableType {
    // Or `self`, `&mut self`
    pub fn methods_on_it(&self) { ... }
}
```

They can be called in Dart:

```dart
var object = await create();
await borrow(object);
await mutable_borrow(object);
await consume(object);
```

## Use the underlying data in Dart

You might wonder, since the Dart side is just a "pointer",
how to use the underlying data in Dart?
The answer is through functions (and/or methods).

In short,
just imagine those fields are private (to Rust) - the **"private fields" concept** you use everyday
to do encapsulation,
then write standard code.

For example, suppose we want to manipulate with the temporary directory object in the sample above,
then the rough code looks like:

```rust
pub struct MyTempDir {
    dir: tempdir::TempDir,
}

impl MyTempDir {
    pub fn new() -> Self { ... }
    
    pub fn directory_path(&self) -> String {
        self.dir.path()
    }
    
    pub fn read_text(&self, filename: String) -> String {
        fs::read_to_string(self.dir.path().join(filename))
    }
    
    // ...
}
```

These methods can be called in Dart as if normal Dart functions (code sketch as below):

```dart
var d = await MyTempDir.newMyTempDir();
print(await d.directoryPath());
print(await d.readText('a.txt'));
```

## Ownership

As expected, the `MyNonEncodableType`, `&MyNonEncodableType`, `&mut MyNonEncodableType`
means the standard Rust ownership things - owned, borrowed, mutable borrowed.
For example, in normal Rust, we cannot mutably borrow the same object twice at the same time.
When doing so for `RustAutoOpaque` objects, you will receive a runtime error.

In short, just write normal Rust code, and you are safe.
Anything that violates Rust's model or safety will be caught and provide a runtime error,
instead of the dangerous undefined behavior.

## Dispose

Every such `RustAutoOpaque` object has a `dispose()` method,
which immediately frees the underlying resource
(tech details if you are worried: indeed decrease the reference count, so dispose-when-use is no problem).
But even if you do not call it,
when Dart does garbage collection (GC),
the same thing will be automatically triggered.

The `dispose` method mimics the standard pattern in Flutter -
we have `dispose` for `ui.Image`, etc.

Should we call `dispose` manually?
This is discussed thoroughly in [this thread](https://github.com/dart-lang/sdk/issues/54233)
and [this related thread](https://github.com/dart-lang/native/issues/848).
For full information, please refer to those posts directly.
In short, thanks to @dcharkes @dnfield @HosseinYousefi @lrhn @mkustermann (ordered alphabetically),
when your underlying Rust objects are huge or takes precious resources (e.g. opens a file),
do manual `dispose` to ensure you release the resource as soon as you do not need them;
otherwise, there is usually no worry about manual dispose calls.

## Force a non-opaque type to be opaque

If a type is indeed encodable, it will by default be translated to the corresponding Dart types.
However, if you want to force it to be opaque, you can use the `#[frb(opaque)]` attribute.

This is useful, for example, when the data is heavy and is mainly used in Rust,
and you do not want to transfer it between Dart and Rust over and over again.

For example:

```rust
struct A { name: String }

#[frb(opaque)]
struct B { name: String }
```

Will generate different Dart code:

```dart
// A pretty standard Dart class with fields inside it
class A { String name; ... }

// A Dart class without data fields, you should pass it to Rust to manipulate it
class B extends RustAutoOpaque {}
```

## Implementation details

As for how it is implemented as well as the design towards safety,
please refer to [this doc](../../contributing/submodules/rust-opaque)
