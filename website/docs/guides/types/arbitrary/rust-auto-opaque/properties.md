# Properties (Accessors)

The `pub` fields of an opaque struct will be automatically translated,
such that it can be used as if it is a normal field.

## Example

Suppose we have the following opaque type:

```rust
pub struct MyOpaqueType {
    pub name: String,
    db: Database,
}
```

Then, the public field, `name`, will be recognized. The getters and setters will be generated:

```dart
// Auto-generated class
class MyOpaqueType {
    String get name => ...;
    set name(String value) => ...;
    ...
}
```

Then, we can use it as if it is a normal field:

```dart
var object = MyOpaqueType();
object.name += 'a';
print('Hi ${object.name}');
```

## Caveats

:::tip
There is no need to memorize anything here (or anything in doc) -
the code generator will provide warnings when detecting non-best-practices.
:::

### Problem description

Because borrowed types are not (yet) supported, the current implementation clones the field when reading it.
This is no problem when the field type is something like integers, Strings, or `RustAutoOpaque<T>`s.
However, it may be confusing in some scenarios.
For example,

```rust
#[frb(opaque)]
pub struct A {
    pub b: B,
}

#[frb(opaque)]
pub struct B {
    pub c: i32,
}
```

Then, usage like

```dart
var a = A(...);
a.b.c += 1;
print(a.b.c); // unchanged
```

may be confusing since the `a.b.c` is not changed.
This is because each access to `a.b` creates a brand new `B` instance.

### Solution 1

One solution is to just add `RustAutoOpaque<...>` like below.
It will not affect other things, for example, the generated type will still be `B`.

```rust
pub struct A {
    pub b: RustAutoOpaque<B>,
}
```

It works because `RustAutoOpaque<T>` is indeed an `Arc`,
thus the cloned `b` will point to the very same object instead of a brand new object.

To create/read/write objects of type `RustAutoOpaque<...>`, please refer to [this page](struct).

### Solution 2

Another way is to make the struct non-opaque (possibly by adding `#[frb(non_opaque)]`). 

### Solution 3

Yet another way is to utilize the [proxy](../../../misc-features/proxy) feature.

This may be the default generated code instead in the future.
If this simplifies your scenario a lot, feel free to create an issue to discuss.
