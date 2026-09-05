# Build for Web

flutter_rust_bridge supports both Dart web compilers: `dart2js` and `dart2wasm`.
The Rust side is WebAssembly in both modes.

| Layer | JavaScript build | WebAssembly build |
| --- | --- | --- |
| Dart or Flutter application | JavaScript from `dart2js` | WasmGC from `dart2wasm` |
| Rust library | WebAssembly from `wasm-pack` | WebAssembly from `wasm-pack` |

## Flutter applications

Build the Rust library before invoking Flutter because Flutter web does not run a Rust build hook.

### Development

Use Dart WebAssembly:

```shell
flutter_rust_bridge_codegen build-web
flutter run -d chrome --wasm \
  --web-header=Cross-Origin-Opener-Policy=same-origin \
  --web-header=Cross-Origin-Embedder-Policy=require-corp
```

Remove `--wasm` to use the JavaScript compiler instead.

### Release

Use Dart WebAssembly:

```shell
flutter_rust_bridge_codegen build-web --release
flutter build web --wasm
```

Use JavaScript:

```shell
flutter_rust_bridge_codegen build-web --release
flutter build web
```

Both Flutter commands write the deployable application to `build/web`.
A Flutter `--wasm` build also contains a JavaScript fallback for browsers without WasmGC support.

## Pure Dart applications

`build-web` can compile the Rust library and the Dart entrypoint together.

Use Dart WebAssembly:

```shell
flutter_rust_bridge_codegen build-web --release \
  --dart-compile-wasm-entrypoint lib/main.dart
```

The default `web` output directory contains:

- **Rust module**: `pkg/<crate>_bg.wasm` and its JavaScript bindings.
- **Dart module**: `main.dart.wasm`.
- **Dart loader**: `main.dart.mjs`.

Load the Dart module from your HTML entrypoint:

```html
<script type="module">
  const { compile } = await import('./main.dart.mjs');
  const response = await fetch('./main.dart.wasm');
  const app = await compile(await response.arrayBuffer());
  const instance = await app.instantiate({});
  instance.invokeMain();
</script>
```

Use JavaScript instead:

```shell
flutter_rust_bridge_codegen build-web --release \
  --dart-compile-js-entrypoint lib/main.dart
```

Then load `main.dart.js` with a normal script element.

Pass both Dart entrypoint options to produce both variants in one invocation.
Unlike Flutter, a pure Dart application must implement its own browser capability detection and fallback selection.

## Compatibility

- **Dart WebAssembly**: Application code and dependencies must support WasmGC and the current JS interop APIs. Replace incompatible imports such as `dart:html` and `package:js` with `package:web` and `dart:js_interop`.
- **Browsers**: Pure Dart Wasm builds require a browser with WasmGC. Flutter Wasm builds select their JavaScript fallback when WasmGC is unavailable.
- **JavaScript runtimes**: Node.js and Deno are not supported execution targets for the flutter_rust_bridge web runtime.
- **64-bit integers**: `i64`, `u64`, `Vec<i64>`, and `Vec<u64>` are supported with both Dart web compilers. Their Dart representation uses `BigInt` and flutter_rust_bridge's `Int64List` or `Uint64List` wrappers.

## Deployment

- Serve the entire output directory; do not deploy only the `.wasm` files.
- Serve `.wasm` files as `application/wasm` and `.mjs` files as JavaScript modules.
- Configure the required cross-origin isolation headers described in [Cross-origin in Web](../../manual/miscellaneous/web-cross-origin).
- Review [Limitations of WASM](../../manual/miscellaneous/wasm-limitations) before depending on threads, panic handling, or browser-specific APIs.

For compiler-specific requirements, see the official [Flutter WebAssembly guide](https://docs.flutter.dev/platform-integration/web/wasm) and [Dart WebAssembly guide](https://dart.dev/web/wasm).
