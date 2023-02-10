# Exceptions

1. For Result/Error, the`anyhow::Result`/`anyhow::Error` is supported. It will be automatically converted to a Dart Exception.
2. For `panic`s, it will also be automatically captured and converted to Dart exceptions.
3. [#582](https://github.com/fzyzcjy/flutter_rust_bridge/pull/582) adds support for error hierarchy, or arbitrary error types. For example, you can create your own `CustomError` (such as using `thiserror`), and it will automatically be converted to a new Dart class. The PR is already almost completed (with the hard work done), and the missing piece is fixing the test errors. I have provided some guidance about how to fix it at the PR comments as well.

## Example

For example, the following code, when called by Dart code, will throw Dart exceptions.

```rust,noplayground
pub fn f() -> anyhow::Result<i32> { bail!("oops I failed") }

pub fn g() -> i32 { panic!("oops I failed") }
```
