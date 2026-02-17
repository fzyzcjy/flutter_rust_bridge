# Creating a new crate

First, if you haven't done so already, create a new crate within your project directory
using `cargo new --lib`. It is recommended that the crate root is a sibling of the other native build
folders for ease of config, e.g.:

```
├── android
├── ios
├── lib
├── linux
├── macos
├── $crate
│   ├── Cargo.toml
│   └── src
├── test
├── web
└── windows
```

Throughout this section we will refer to your crate name as $crate. Unless otherwise noted, the crate folder
and the crate name will be used interchangeably.

Next, add these two lines to your `Cargo.toml`:

```diff
+[lib]
+crate-type = ["staticlib", "cdylib"]
```

This configures your crate to be output as a static library for MacOS and iOS,
and a dynamic library on other platforms. Configure this to your needs.
If you would like to write tests or benchmarks, append `"rlib"` to the list
as well.
