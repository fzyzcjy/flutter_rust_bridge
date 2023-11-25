import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// Load the [ExternalLibrary], with the following cases in mind:
/// 1. When `flutter run`, or when a real app is bundled.
/// 2. When running Flutter widget tests.
/// 3. When `dart test`, `dart run`, `dart compile exe`, etc.
ExternalLibrary loadExternalLibrary({
  String? defaultExternalLibraryRelativeDirectory,
  required String stem,
}) {
  return loadExternalLibraryRaw(
    nativeLibDirWhenNonPackaged: defaultExternalLibraryRelativeDirectory == null
        ? null
        : Directory.current.uri.resolve(defaultExternalLibraryRelativeDirectory),
    stem: stem,
  );
}

ExternalLibrary loadExternalLibraryRaw({
  Uri? nativeLibDirWhenNonPackaged,
  required String stem,
}) {
  // ref
  // * https://flutter.dev/docs/development/platform-integration/c-interop
  // * https://github.com/fzyzcjy/flutter_rust_bridge/pull/898

  ExternalLibrary? tryAssumingNonPackaged(String name) {
    if (nativeLibDirWhenNonPackaged == null) return null;
    final filePath = nativeLibDirWhenNonPackaged.resolve(name).toFilePath();
    if (!File(filePath).existsSync()) return null;
    return DynamicLibrary.open(filePath);
  }

  if (Platform.isAndroid) {
    return DynamicLibrary.open('lib$stem.so');
  }

  if (Platform.isIOS) {
    return DynamicLibrary.process();
  }

  if (Platform.isWindows) {
    final name = '$stem.dll';
    return tryAssumingNonPackaged(name) ?? DynamicLibrary.open(name);
  }

  if (Platform.isMacOS) {
    final name = 'lib$stem.dylib';
    return tryAssumingNonPackaged(name) ?? DynamicLibrary.process();
  }

  if (Platform.isLinux) {
    final name = 'lib$stem.so';
    return tryAssumingNonPackaged(name) ?? DynamicLibrary.open(name);
  }

  // Feel free to PR to add support for more platforms! (e.g. I do not have a Fuchsia device, so cannot test that)
  throw Exception('loadExternalLibrary failed: Unknown platform=${Platform.operatingSystem}');
}
