# Generating bindings

Now that we've got most of the plumbing out of the way, let's compile our Rust
application. If you just created your crate a few moments ago, go ahead and
add a new file at `$crate/src/api.rs` and replace its contents with this snippet or
whatever suits your fancy:

```rust,ignore
pub fn greet() -> String {
    "Hello from Rust! 🦀".into()
}
```

then in `$crate/src/lib.rs`:

```diff
+mod api;
```

## Running the codegen

Before we can compile the library, we need to generate the bindings first.
From the root of the app, run these commands:

```bash
{{#include command.sh.txt}}
```

> **Note:** These will be the same commands to use whenever you modify your Rust library code.

Running this command yields the C header of the functions and types exported
by the Rust library, which we will need to keep the symbols from being stripped.
