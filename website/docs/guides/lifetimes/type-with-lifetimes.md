# Types with lifetimes

We can have function like `fn f<'a>(foo: &'a Foo) -> Bar<'a>`,
i.e. returning a type with lifetime on it.

There are some syntax rules to follow because we have not implemented logic about lifetime elision or fancy lifetime parsing.
If some rules make your scenario very complicated, feel free to create an issue to discuss.

* Specify lifetimes explicitly. For example, change `fn f(foo: &Foo) -> Bar` to `fn f<'a>(foo: &'a Foo) -> Bar<'a>`. This will not affect code correctness.
* Only one lifetime specifier name (can have other unnamed specifiers). For example, `fn f<'a>(a: &A, b: &'a B, c: &C<'a>, d: &D<'_>) -> E<'a>` is acceptable.

(Optional) If you are worried whether you need to manually keep the Dart `Foo` object live longer than `Bar`,
the answer is - no need.
You can safely call Dart `Foo.dispose` or let Dart `Foo` be garbage collected, and the `Bar` will still be valid.
Indeed, `Bar` internally ensures that the Rust `Foo` object lives longer.

## Example

Suppose we have the following Rust code:

```rust
#[frb(opaque)]
pub struct Foo(String);

// For simplicity and demonstration, we use a field pointing to Foo here; but this struct indeed can be anything
#[frb(opaque)]
pub struct Bar<'a> { foo: &'a Foo }

impl Foo {
    pub fn compute_bar<'a>(&'a self) -> Bar<'a> { .. }
}

impl Bar<'_> {
    pub fn greet(&self) { .. }
}
```

Then, we can use it like:

```dart
var foo = Foo(..);
var bar = foo.computeBar();
bar.greet();
```

