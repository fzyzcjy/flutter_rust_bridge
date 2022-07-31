# Overall design

> This doc is still WIP. Tracking issue: https://github.com/fzyzcjy/flutter_rust_bridge/issues/593

## Folder structure

* `frb_codegen`: Code generator. It inputs `api.rs` and outputs Rust and Dart code files.
* `frb_example`: Examples.
  * `pure_dart`: Not only an example, but, more importantly, serves as end-to-end tests.
  * `with_flutter`: Example with integration into Flutter.
  * `pure_dart_multi`: Demonstrate multi-file usage.
* `frb_dart`: Support library for Dart - to be imported by users.
* `frb_rust`: Support library for Rust - to be imported by users.
* `frb_macros`: Indeed part of `frb_rust`. <small>It is a separate package simply because limitation of proc macros.</small>
* `book`: The documentation.
* `.github`: GitHub-related.
  * `workflows/ci.yaml`: Definition of CI workflows.

## TODO

TODO
