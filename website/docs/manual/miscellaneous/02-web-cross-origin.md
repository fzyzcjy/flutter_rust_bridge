# Cross-origin in Web

## Background

When using Rust (WASM) and Flutter on the web platform,
the web server needs to respond with the following headers:

- `Cross-Origin-Opener-Policy`: `same-origin`
- `Cross-Origin-Embedder-Policy`: `credentialless` OR `require-corp` (Safari seems to need `require-corp`)

## When `flutter run`

### After Flutter 3.17

A [pull request](https://github.com/flutter/flutter/pull/136297) has already been merged into Flutter in 2023 Oct.
Then, doing it is as easy as:

```shell
flutter run \
    --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
```

### Before Flutter 3.17

Temporarily (before >=3.17), the Flutter source code installed on your computer needs to be hacked as follows.

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

## When deploy

Please refer to the web server you are using to see how to add these HTTP headers.
