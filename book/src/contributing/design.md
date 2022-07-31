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

## Code-generator structure

The pipeline is as follows:

```
----------    src/parser    ----------    src/generator     ---------------
| api.rs | ---------------> | src/ir | -------------------> | Rust & Dart |
----------                  ----------                      ---------------
```

* The input, `api.rs` in the figure, is the user-provided handwritten Rust code.
* The parser (`src/parser`) converts the input code (indeed [syn](https://crates.io/crates/syn) tree) into IR.
* IR (`src/ir`), or internal representation, is a data structure that represents the information of the code that we are interested in.
* The generator (`src/generator`) converts the IR into final outputs. More specifcially, as you can probably guess, `src/generator/dart` generates Dart code, `src/generator/rust` is for Rust code, and `src/generator/c` is for (a bit of) C code.
* The outputs (`Rust & Dart` in the figure) are written to corresponding files.

## Data flow

### Dart -> Rust

TODO: a figure

TODO: api2wire, wire2api

### Rust -> Dart

TODO: a figure

TODO: api2wire, wire2api

### A function call

TODO

## Data copies

TODO: normal case

TODO: ZeroCopyBuffer and its impl
