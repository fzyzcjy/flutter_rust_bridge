# Cross-origin in Web

## Background

When using Rust (WASM) and Flutter on the web platform,
the web server needs to respond with the following headers to enable shared buffers:

- `Cross-Origin-Opener-Policy`: `same-origin`
- `Cross-Origin-Embedder-Policy`: `credentialless` OR `require-corp` (Safari seems to need `require-corp`)

Cross-origin isolated documents have less restrictions on advanced features, such as asynchronous WASM which utilize shared buffers.
You can read more about crossOriginIsolation [here](https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated).

## When `flutter run`

Thanks to [this pull request](https://github.com/flutter/flutter/pull/136297), we can:

```shell
flutter run --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
```

<details>
<summary>When using Flutter &lt; 3.17</summary>

If you are still using Flutter before 3.17, the Flutter source code installed on your computer needs to be hacked as follows.

Suppose your `flutter` is installed at `/whatever-path/bin/flutter` (this can be found by e.g. `which flutter`).
Firstly, modify the file at `/whatever-path/packages/flutter_tools/lib/src/isolated/devfs_web.dart`.
Find out the line

```dart
httpServer!.defaultResponseHeaders.remove('x-frame-options', 'SAMEORIGIN');
```

And *add* a few lines about headers near it:

```diff
     httpServer!.defaultResponseHeaders.remove('x-frame-options', 'SAMEORIGIN');
 
+    print('Temporary hack Flutter framework to add headers');
+    httpServer!.defaultResponseHeaders.add('cross-origin-opener-policy', 'same-origin');
+    httpServer!.defaultResponseHeaders.add('cross-origin-embedder-policy', 'require-corp');
+
     final PackageConfig packageConfig = buildInfo.packageConfig;
```

Secondly, you need to remove the following file to let Flutter understand the source has been changed.

```shell
rm /whatever-path/bin/cache/flutter_tools.stamp
```

</details>

## When `flutter drive`

Thanks to [this pull request](https://github.com/flutter/flutter/pull/136297), we can do something similar:

```shell
flutter drive --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
```

## Use without cross-origin headers
While not recommended, it is possible to use FRB WASM without using cross-origin isolation.
Such as in rare cases where HTTPS hosting is not an option.
In these cases most browsers (for security reasons) will ignore cross-origin headers unless a valid HTTPS connection is made.
To avoid this we can restrict ourselves to only using the main thread, and making sure we don't try and spawn any more.
This will stop the need for shared buffers, and likewise will not need the cross-origin headers sent by the web server.

In rust
- `pub async fn`: On main thread (when in web)
- `#[frb(sync)] pub fn`: On main thread
- `pub fn`: NOT on the main thread by default

Alternatively use option `default_dart_async: false` to avoid annotating each function with `#[frb(sync)]`

## When deploy
Please refer to the web server you are using to see how to add these HTTP headers.
