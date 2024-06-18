# Flutter wrapper
On this page, we will start creating the Flutter wrapper around our Dart-only library package.
We start with the plugin_ffi Flutter template since it is somewhat similar to what we need,
but we will need to modify it significantly in the coming steps.
Configuring the build processes for each supported platform is also a bit involved,
so those are covered individually in the coming pages.

Run `flutter create --help` to see all the available options; you may want to set some (like `--org`).

Finally, in the `packages` folder, run the following, adding any other options you choose
and replacing `library_name` with your library name:
```bash
flutter create --template=plugin_ffi --platforms=android,ios,macos,linux,windows --org=com.example flutter_library_name
```

## Additional setup steps
1. Add your Dart-only base as a dependency in your new Flutter package's `pubspec.yaml`.
*Use the version syntax, e.g. `^1.0.0`*. Melos will take care of the dependency resolution for us.
2. If you choose to have integration testing in CI (recommended),
add an `integration_test` folder to your Flutter package's and/or Flutter example package's root directory,
then add the following to the  `pubspec.yaml` of the applicable package(s):
```yaml
dev_dependencies:
  flutter_test:
    sdk: flutter
  integration_test:
    sdk: flutter
```
3. In `/packages/flutter_library_name/lib/flutter_library_name.dart`,
add the following near the top of the file, replacing `library_name` with your Dart-only package's name:
```dart
export 'package:library_name/library_name.dart';
```
This re-exports your Dart-only package to users of your Flutter package,
so they only need to do one `flutter pub add`.

4. Finally, we will need to write some code to be able to handle FFI in Flutter.
Modify the following as needed (replacing `library_name` and `LibraryName` with your library name).
```dart
// lib/src/ffi/stub.dart
Object createLibraryImpl() => throw UnimplementedError();
```
```dart
// lib/src/ffi/io.dart
import 'dart:ffi';
import 'dart:io';

DynamicLibrary createLibraryImpl() {
  const base = 'library_name';

  if (Platform.isIOS || Platform.isMacOS) {
    return DynamicLibrary.open('$base.framework/$base');
  } else if (Platform.isWindows) {
    return DynamicLibrary.open('$base.dll');
  } else {
    return DynamicLibrary.open('lib$base.so');
  }
}
```
```dart
// lib/src/ffi/web.dart
import 'package:library_name/library_name.dart';

WasmModule createLibraryImpl() {
  // TODO add web support. See:
  // https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter/lib/ffi.web.dart
  throw UnsupportedError('Web support is not provided yet.');
}
```
```dart
// lib/src/ffi.dart
import 'package:library_name/library_name.dart';
import 'ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.js_interop) 'ffi/web.dart';

LibraryName createLib() =>
    createWrapper(createLibraryImpl());
```
5. Run `melos bs`

Now, inside your Flutter library, you can call `createLib()` to get an instance of the FRB-generated Dart class!
However, it won't work just yet; we will wire up our Flutter package to use our Rust binaries in the next subsection.
