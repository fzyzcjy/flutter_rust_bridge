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

## Debugging on Flutter Web

When you want to debug by launching `flutter run -d chrome` (or the equivalent in the code editor of your choice),
a web socket server is set up by that process the debugger can connect to. This operation can't really be replaced
by serving some static files from a web server as described in the previous section.

So, you need a bit more of a setup. It's possible to get this working by writing a reverse proxy that forwards all
requests to the Flutter's development server, but adding the aforementioned HTTP headers. Unfortunately, it's not
possible to configure the official development server to just add these headers directly.

Here is an example reverse proxy written in Rust using the hyper crate:

```rs
use std::str::FromStr;

use warp::{
    http::HeaderValue,
    hyper::{
        header::{
            HeaderName, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
            ACCESS_CONTROL_ALLOW_ORIGIN,
        },
        Body, Response,
    },
    Filter, Rejection, Reply,
};
use warp_reverse_proxy::reverse_proxy_filter;

async fn add_headers(mut response: Response<Body>) -> Result<impl Reply, Rejection> {
    let headers = response.headers_mut();
    headers.insert(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_str("http://localhost:3000").unwrap(),
    );
    headers.insert(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_str("true").unwrap(),
    );
    headers.insert(
        ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_str("content-type, accept, authorization").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("cross-origin-opener-policy").unwrap(),
        HeaderValue::from_str("same-origin").unwrap(),
    );
    headers.insert(
        HeaderName::from_str("cross-origin-embedder-policy").unwrap(),
        HeaderValue::from_str("require-corp").unwrap(),
    );

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    warp::serve(
        reverse_proxy_filter("".to_owned(), "http://localhost:3001".to_owned())
            .and_then(add_headers),
    )
    .run(([127, 0, 0, 1], 3000))
    .await;

    Ok(())
}
```

Your Cargo.toml needs the following dependencies (update to the latest version when available):

```toml
tokio = { version = "1.24.2", features = ["rt-multi-thread", "process", "macros"] }
warp = "0.3.3"
warp-reverse-proxy = { version = "1.0.0", default-features = false, features = ["rustls-tls"] }
```

This server opens port 3000 and forwards all requests to a server running on port 3001 (these
port numbers are just arbitrary picks, you can chose any other pair > 1024).

In order to point the web app to the right websocket port, you have to modify `web/index.html`.
Locate the `<script>` tag inside the `<body>`, and right after it, add the following lines:

```js
Object.defineProperty(window, "$dwdsDevHandlerPath", {
    value: 'ws://localhost:3001/$dwdsSseHandler',
    writable: false
});
```

Next, you need to tell the flutter command to listen on port 3001, but open up port 3000.
If you skip the latter part, all absolute URLs on the generated pages will point towards the
unproxied port and thus will fail.

The command for this is

```sh
flutter run -d chrome --web-port 3001 --web-launch-url http://localhost:3000/
```

### Canvaskit

You're also going to need to serve a local canvaskit.wasm due to security restrictions when using
these headers (can't load wasm modules from a different origin). When you build your application
for production on the web platform, the necessary files will be included on the path
`build/web/canvaskit/canvaskit.{js,wasm}`. Copy them to `web/canvaskit/` (creating the `canvaskit`
directory) and modify `web/index.html` as follows:

Locate the `onEntrypointLoaded:` definition of the `loadEntrypoint` function call and replace it
with:

```js
onEntrypointLoaded: async function (engineInitializer) {
    const appRunner = await engineInitializer.initializeEngine({
        canvasKitBaseUrl: '/canvaskit/',
    });
    appRunner.runApp();
}
```

This should be enough to debug on the Web with flutter_rust_bridge! After this rather tedious setup,
during regular development you only need to run the reverse proxy server and launch Flutter's
development server as normal, so it's very easy to use afterwards.
