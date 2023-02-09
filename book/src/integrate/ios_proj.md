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
Don't open the project yet; we need to configure it through the parent projects first.
