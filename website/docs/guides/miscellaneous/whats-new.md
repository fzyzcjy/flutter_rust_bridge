# What's new in V2

## Changelog

:::tip
For a more vivid illustration, I suggest glancing through [the homepage](../../../).
:::

### Primary changes

* Transparently handle arbitrary Rust types, even if they are not serializable or non-clone.
* Support asynchronous Rust, i.e. `async fn` (e.g. Tokio)
* Allow Rust to call Dart functions (previously only allow Dart to call Rust)
* Out-of-the-box usage - setup the whole system within a few clicks.
    * Add scaffolding commands (apply to existing project or create new project in one command).
    * Integrate with Cargokit (setup the build environment and hooks automatically).
    * Provide defaults, while keeping customizability (e.g. auto find and load the dynamic libraries by default, while allowing full manual control).
* Support multiple input and output files (indeed can generate for a whole crate, preserving file hierarchy)
* Can directly use `flutter run` for all platforms (including web).
    * Therefore, support web debugging, firefox, etc.
* A new codec, `SSE`, which is several times faster under typical workload

### More changes

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
* Improve error stack trace of `frb_codegen` (replace `thiserror` with `anyhow`).
* Generate hashCode/equals for non-freezed structs.
* Generate hashCode/equals for non-freezed structs.
* Remove global internal thread pool, thus users can arbitrarily customize thread pool easily.
* Refactor the scripts for repository contributors and the CI.
* Automatically install `cargo expand` if not installed.
* Fix bugs.

## Upgrade guide

To upgrade, please refer to [the upgrade guide](upgrade).
