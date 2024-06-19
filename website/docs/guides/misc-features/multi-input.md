# Multiple input folders

The `rust_input` configuration key supports multiple entries separated by commas.
For example, consider the following configuration:

```yaml
rust_input: crate::api,crate::hello::world
```

Roughly speaking, it will scan `src/api/**/*.rs` and `src/hello/world/**/*.rs`.

More strictly speaking, it scans Rust modules instead of real files, thus complex scenarios such as multiple modules
inside one file are supported.
