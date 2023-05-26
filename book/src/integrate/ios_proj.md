# Creating the Rust project

First, follow the instructions on the [Usage](https://gitlab.com/kornelski/cargo-xcode#usage)
section of `cargo-xcode`. The instructions that follow are quoted from there, but keep in mind
that it might have become outdated.

---

Ensure that these lines are present in your `$crate/Cargo.toml`:

```toml
[lib]
crate-type = ["lib", "staticlib", "cdylib"]
```

where

- `lib` is required for non-library targets, such as tests and benchmarks
- `staticlib` is required for iOS
- `cdylib` for all other platforms

Configure this to suit your needs. Then run this command in `$crate`:

```bash
cargo xcode
```

This will generate a `$crate/$crate.xcodeproj` that can be imported into other Xcode projects.
You only have to do this once per crate.

Now, open up that `$crate/$crate.xcodeproj` file with Xcode and select the root item at the left pane. The item's name will be identical to your crate's name. In the **Build Settings** tab, search for `Dynamic Library Install Name Base` and change the value into `@executable_path/../Frameworks/`. This is [required by cargo-xcode](https://lib.rs/crates/cargo-xcode#:~:text=DYLIB_INSTALL_NAME_BASE) to enable macOS executable to properly locate `.dylib` library files in the package.
