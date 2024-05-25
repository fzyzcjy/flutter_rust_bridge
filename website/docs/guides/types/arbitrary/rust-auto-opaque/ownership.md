# Ownership

As expected, the `MyNonEncodableType`, `&MyNonEncodableType`, `&mut MyNonEncodableType`
means the standard Rust ownership things - owned, borrowed, mutable borrowed.
For example, in normal Rust, we cannot mutably borrow the same object twice at the same time.
When doing so for `RustAutoOpaque` objects, you will receive a runtime error.

In short, just write normal Rust code, and you are safe.
Anything that violates Rust's model or safety will be caught and provide a runtime error,
instead of the dangerous undefined behavior.
