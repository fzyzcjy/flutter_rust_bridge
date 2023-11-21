import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// Load the [ExternalLibrary], with the following cases in mind:
/// 1. When `flutter run`, or when a real app is bundled.
/// 2. When running Flutter widget tests.
/// 3. When `dart test`, `dart run`, `dart compile exe`, etc.
ExternalLibrary loadExternalLibrary({
  Uri? nativeLibDirWhenNonPackaged,
  required String stem,
}) {
  // ref
  // * https://flutter.dev/docs/development/platform-integration/c-interop
  // * https://github.com/fzyzcjy/flutter_rust_bridge/pull/898

  // TODO impl

  if (Platform.isAndroid) {
    return DynamicLibrary.open('lib$stem.so');
  }

  if (Platform.isIOS) {
    return DynamicLibrary.process();
  }

  if (Platform.isWindows) {
    return DynamicLibrary.open('$stem.dll');
  }

  if (Platform.isMacOS) {
    return DynamicLibrary.process();
  }

  if (Platform.isLinux) {
    return DynamicLibrary.open('lib$stem.so');
  }

  // Feel free to PR to add support for more platforms! (e.g. I do not have a Fuchsia device, so cannot test that)
  throw Exception('loadExternalLibrary failed: Unknown platform=${Platform.operatingSystem}');
}
