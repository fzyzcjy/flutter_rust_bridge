# Limitations of WASM

- Safari cannot spawn nested `Worker`s. A workaround is to build two variants of the library,
  one with multithreading and one without, and serve Safari users the single-threaded variant.
  For a more general solution, check out [wasm-feature-detect](https://github.com/GoogleChromeLabs/wasm-feature-detect)
  or [nested-worker](https://github.com/johanholmerin/nested-worker).
- `std::thread::spawn` is unimplemented and replacements (e.g. `wasm_thread`) are not fully supported. 
  If you must use them, consider wrapping your return type in a `SyncReturn<_>` to
  avoid the internal thread pool interfering with your threads.
  This library includes a `spawn!` macro which spawns a new thread using the internal thread pool.
- When a Rust thread panics, it aborts and throws a JavaScript `RuntimeError` that cannot be caught by name in
  Dart. This is expected to change as the exception handling story for WASM improves, but a rule of thumb
  is to replace `.unwrap` with `.expect` or `Err`s.
- As a consequence, `panic::catch_unwind` does not work on the Web. As of writing, the implementation to
  catch these errors resides within the bodies of the workers, i.e. it is not straightforward enough to
  generalize for other use-cases.
- `Int64List` and `Uint64List` throws when used on Web platforms. They are left intentionally
  unimplemented by the Dart language developers, perhaps due to the differences between `int` and `BigInt`.
  This library provides a barebones pure Dart shim whose behavior may differ from the specifications,
  so please create an issue/PR if you encounter any significant digression.
- `Int64List` and `Uint64List` arithmetics clamp on native platforms, but wrap on the Web. If your use-case
  requires precision around large integer values, please be mindful of these platform-specific differences.
- Support for the various components of WASM is not universal among browsers. Here is a (non-exhaustive) list
  of trackers for how widely available some of the features are across browsers:
  - [`MessagePort` error events](https://caniuse.com/mdn-api_messageport_messageerror_event)
  - [`crossOriginIsolated`](https://caniuse.com/mdn-api_crossoriginisolated)
  - [Shared Array Buffers](https://caniuse.com/sharedarraybuffer)
  - [`BroadcastChannel`](https://caniuse.com/sharedarraybuffer)
  - [Atomics](https://caniuse.com/mdn-javascript_builtins_atomics)
  - [`BigInt64Array`](https://caniuse.com/mdn-javascript_builtins_bigint64array)
  - [WebAssembly](https://caniuse.com/wasm)
  - [WebAssembly roadmap](https://webassembly.org/roadmap/)
- JavaScript runtimes (Node.js, Deno, etc.) support is not yet implemented.
