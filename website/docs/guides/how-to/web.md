# Build for Web

flutter_rust_bridge supports `dart2js` and `dart2wasm`. Rust compiles to WebAssembly in both cases.

## Flutter applications

Build Rust first, then run Flutter:

```shell
flutter_rust_bridge_codegen build-web
flutter run -d chrome --wasm \
  --web-header=Cross-Origin-Opener-Policy=same-origin \
  --web-header=Cross-Origin-Embedder-Policy=require-corp
```

For release:

```shell
flutter_rust_bridge_codegen build-web --release
flutter build web --wasm
```

- Omit `--wasm` to compile Dart to JavaScript.
- Deploy `build/web`. Wasm builds include a JavaScript fallback for browsers without WasmGC.

## Pure Dart applications

Build Rust and Dart together:

```shell
flutter_rust_bridge_codegen build-web --release \
  --dart-compile-wasm-entrypoint lib/main.dart
```

The default output is `web/`: Rust artifacts in `pkg/`, plus `main.dart.wasm` and `main.dart.mjs`.
Load Dart from your HTML entrypoint:

```html
<script type="module">
  const { compile } = await import('./main.dart.mjs');
  const response = await fetch('./main.dart.wasm');
  const app = await compile(await response.arrayBuffer());
  const instance = await app.instantiate({});
  instance.invokeMain();
</script>
```

- For JavaScript, use `--dart-compile-js-entrypoint lib/main.dart` and load `main.dart.js` with a script element.
- Both options can be combined; browser fallback selection is up to your application.

## Compatibility

- Dart Wasm requires WasmGC and compatible dependencies. Use `package:web` and `dart:js_interop`, not `dart:html` or `package:js`.
- See the [Flutter](https://docs.flutter.dev/platform-integration/web/wasm) and [Dart](https://dart.dev/web/wasm) compiler requirements and [FRB limitations](../../manual/miscellaneous/wasm-limitations).

## Deployment

- Serve the entire output directory.
- Serve `.wasm` files as `application/wasm` and `.mjs` files as JavaScript modules.
- Configure [cross-origin headers](../../manual/miscellaneous/web-cross-origin) for both Dart compilers.
