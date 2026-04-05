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

## Run without cross-origin headers

While not recommended, it is possible to use flutter_rust_bridge Web (WASM) without using cross-origin isolation
by following the steps below.

When will this be needed:
For example, when we are in rare cases where HTTPS hosting is not an option, 
most browsers will ignore cross-origin headers for security reasons.

The main idea is to restrict ourselves to only use the main thread and never use other threads,
which will stop the need for shared buffers, thus removing the need of cross-origin headers.

To achieve this, simply configure `default_dart_async: false` (which makes every `pub fn` automatically `#[frb(sync)] pub fn`), and then use `pub fn / pub async fn` as usual.

Note that Rust's async system, such as `pub async fn` and [async spawns](../../guides/cross-platform/async), still works well here, because it runs on the main thread by default on the Web platform (though it is multi-threaded on native platforms).

## When deploy

Please refer to the web server you are using to see how to add these HTTP headers.
