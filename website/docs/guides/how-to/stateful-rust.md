# Stateful Rust

What if our Rust code needs to be stateful?
Here are some non-exhaustive possibilities using the flutter_rust_bridge features.

### Automatic arbitrary Rust types

The [automatic arbitrary Rust types](../types/arbitrary/rust-auto-opaque)
can be used to hold the state -
just write normal Rust code, and use `#[frb(opaque)]` on your struct to ensure it is
never serialized and transferred across the language boundary.

I personally prefer this approach, because it is the most seamless,
and avoid introducing any global variables (we know global variables do not have a very good reputation).

Please refer to gallery demo for some in-action examples.

### Alternative: `lazy_static`

The `lazy_static`, or any other static variable solution in Rust,
can also be used to hold states in Rust.
This is a Rust feature and is unrelated to flutter_rust_bridge.

### Alternative: Copy data

If the state is quite small, it may also be a choice to transfer the state back and forth
between Rust and Dart.
This is again automatically done, as long as your struct is recognized as non-opaque.
