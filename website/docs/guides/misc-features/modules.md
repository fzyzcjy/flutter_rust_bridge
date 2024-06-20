# Modules

:::info
Currently, only commonly used module syntaxes are supported
(feel free to create an issue if you need something not implemented yet).
When implementing more sophisticated module grammar parser,
it is possible to have breaking changes caused by improved understanding of modules not following semver.
However, such changes are usually trivial to migrate.
:::

## Support of modules

Since `flutter_rust_bridge` utilizes cargo-expand to understand the source code,
many Rust module grammar are supported automatically.
For example, `mod something;` and `mod another_inline_module { ... }` are both allowed.

## Support of `pub use`

It is supported to have code like:

* `pub use a::b::c;` (normal imports)
* `pub use a::b::*;` (wildcard imports)
* `pub use a::{b, x::{y,z}};` (multiple imports even with such nesting)
