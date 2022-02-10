# Creating a new crate

First, create a new crate within your project directory using `cargo new --lib`.
It is recommended that the crate root is a sibling of the other native build
folders for ease of config.

Next, add these two lines to your `Cargo.toml`:

```diff
+[lib]
+crate-type = ["staticlib", "cdylib"]
```

This configures your crate to be output as a static library for MacOS and iOS,
and a dynamic library on other platforms. Configure this to your needs.
If you would like to write tests or benchmarks, append `"rlib"` to the list
as well.