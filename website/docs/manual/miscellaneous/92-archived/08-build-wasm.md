# Building a WASM binary manually

Here are the complete commands for building a WASM binary with this library:

```bash
export RUSTUP_TOOLCHAIN=nightly
export RUSTFLAGS="-C target-feature=+atomics,+bulk-memory,+mutable-globals"
wasm-pack build \
    -t no-modules \
    -d <WASM_OUTPUT_PATH> \
    --no-typescript -- \
    -Z build-std=std,panic_abort
```

Continue reading for more details.

---

`flutter_rust_bridge_codegen` expects a certain setup that is modeled after the
[wasm_bindgen raytracing example](https://github.com/rustwasm/wasm-bindgen/tree/main/examples/raytrace-parallel)
and by extension consumes the [`wasm_bindgen`] library and its ecosystem.
The requirements are:
- The standard library being built with the `panic_abort` feature
- The library and standard library being built with the target
  features `atomics`, `bulk_memory` and `mutable_globals`
- `wasm-pack` called with `-t no-modules` (to be relaxed in the future)

Note that these features also represent a hard requirement on your
users' browser versions.

Furthermore, this library does not support JavaScript
runtimes as of writing.

**WASM_OUTPUT_PATH** refers to the output directory of the WASM module.
If running Flutter, this is usually `web/pkg`.

## Setting up the web server

Once you have built your binary and are ready to deploy, you will also need
to configure your web server to respond with these two headers:
- [`Cross-Origin-Resource-Policy`] set to `same-origin`
- [`Cross-Origin-Embedder-Policy`] set to `require-corp`[^1]

Here is a sample web server that accomplishes this task (excerpt from `flutter_rust_bridge_serve`):

```dart
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart';
import 'package:shelf_static/shelf_static.dart';

void main() async {
    final root = "/* directory containing index.html */";
    final staticFilesHandler = createStaticHandler(root, defaultDocument: 'index.html');
    final handler = const Pipeline().addMiddleware((handler) {
        return (req) async {
            final res = await handler(req);
            return res.change(headers: const {
                'Cross-Origin-Opener-Policy': 'same-origin',
                'Cross-Origin-Embedder-Policy': 'require-corp',
            });
        };
    }).addHandler(staticFilesHandler);
    await serve(handler, 'localhost', 8080);
}
```

[^1]: When running Flutter Web, you may encounter issues with downloading Flutter
      support scripts which have not been marked as `crossorigin="anonymous"` and
      therefore cannot be loaded. For local testing, you can specify `credentialless`
      instead.

[`wasm_bindgen`]: https://rustwasm.github.io/docs/wasm-bindgen
[`Cross-Origin-Resource-Policy`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Cross-Origin_Resource_Policy_(CORP)
[`Cross-Origin-Embedder-Policy`]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy