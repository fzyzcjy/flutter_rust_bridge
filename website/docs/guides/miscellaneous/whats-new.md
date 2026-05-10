# What's new in V2

## Upgrade guide

To upgrade, please refer to [the upgrade guide](upgrade/v2).

## Changelog

:::tip
For a more vivid illustration, I suggest glancing through [the homepage](https://github.com/fzyzcjy/flutter_rust_bridge),
where there are lists of features (advantages) and a quick show-me-the-code.
:::

### Primary changes

* From 1.x to 2.0.0-dev.0:
  * **Rapid setup**: Only a one-liner command to integrate into your project.
  * **Arbitrary types**: Use arbitrary Rust and Dart types without manual intervention, even if they are not serializable or non-clone (previously need some manual intervention).
  * **Async Rust**: Support asynchronous Rust (`async fn`), in addition to sync Rust / async Dart / sync Dart.
  * **Rust call Dart**: Allow Rust to call Dart functions (previously only allow Dart to call Rust).
  * **Support whole folders as inputs**: Previously only support one single file (e.g. `api.rs`).
  * **Use libraries/tools in Flutter/Rust**: All existing libraries, Flutter debuggers, ... Nothing to stop you from using them.
* From 2.0.0-dev.0 to 2.0.0:
  * **Parsing third-party packages**: Scan and use existing Rust packages in Dart (experimental).
  * **Lifetimes**: Support returning types with lifetime specifiers (experimental).
  * **Traits**: Support traits as base classes and trait objects.
  * **New codec**: A new codec, `SSE`, which is several times faster under some workloads.
  * **Others (>200 PRs)**: Auto and manual accessors, object proxies, user-defined serializers, developer experience, deadlock-free auto locking, Rust initializers, included batteries, renaming and ignoring, improving streams, more types, ...

### More changes

#### From 1.x to 2.0.0-dev.0

<details>

* Overhaul the whole internal codebase
    * Making it clear / modualized / elegant, enabling quick development of future features
    * Unsafe code are further mimimalized and encapsulated
    * Details are omitted here, since it does not affect end users reading this CHANGELOG.md
* Reduce the dependency of the `flutter_rust_bridge` package to the bare minimum.
* Refactor the APIs, making it more developer friendly.
* Refactor the documentation.
* Refactor the tests.
    * Add sanitizers (ASAN, LSAN, ...) in addition to existing Valgrind into CI.
    * Automatically triple the tests, e.g. can auto ensure Rust-async works well since it has the same number of tests as the Rust non-async mode.
    * Automatically generate some e2e exhaustive test cases.
    * Add some unit tests for `frb_codegen`.
* Details of out-of-the-box usage
  * Add scaffolding commands (apply to existing project or create new project in one command).
  * Integrate with Cargokit (setup the build environment and hooks automatically).
  * Provide defaults, while keeping customizability (e.g. auto find and load the dynamic libraries by default, while allowing full manual control).
* Improve error stack trace of `frb_codegen` (replace `thiserror` with `anyhow`).
* Generate hashCode/equals for non-freezed structs.
* Remove global internal thread pool, thus users can arbitrarily customize thread pool easily.
* Refactor the scripts for repository contributors and the CI.
* Automatically install `cargo expand` if not installed.
* Fix bugs.

</details>

#### From 2.0.0-dev.0 to 2.0.0

Please refer to the CHANGELOG for >200 PRs.
