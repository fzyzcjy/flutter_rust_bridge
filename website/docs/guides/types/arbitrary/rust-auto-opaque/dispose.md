# Dispose

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
