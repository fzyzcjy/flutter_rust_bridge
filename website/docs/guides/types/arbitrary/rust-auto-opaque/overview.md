# Overview

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

P.S. As for how it is implemented as well as the design towards safety,
please refer to [this doc](../../contributing/submodules/rust-opaque)

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
