# Overview

For each of Rust and Dart, we support the synchronous and asynchronous grammar.
The combinations, with brief explanations of its semantics, are listed as follows:

* **Async Dart + Sync Rust**: Dart is non-blocking, Rust uses thread pool
* **Async Dart + Async Rust**: Dart is non-blocking, Rust uses async runtime
* **Sync Dart + Sync Rust**: Dart is blocking, Rust is executed on main thread
* ~~Sync Dart + Async Rust~~ (not very reasonable)
