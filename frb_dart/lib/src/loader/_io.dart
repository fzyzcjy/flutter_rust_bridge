import 'dart:async';
import 'dart:io';

import 'package:flutter_rust_bridge/src/loader/_common.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// Load the [ExternalLibrary], with the following cases in mind:
/// 1. When `flutter run`, or when a real app is bundled.
/// 2. When running Flutter widget tests.
/// 3. When `dart test`, `dart run`, `dart compile exe`, etc.
FutureOr<ExternalLibrary> loadExternalLibrary(
    ExternalLibraryLoaderConfig config) async {
  final ioDirectory = config.ioDirectory;
  return loadExternalLibraryRaw(
    nativeLibDirWhenNonPackaged:
        ioDirectory == null ? null : Directory.current.uri.resolve(ioDirectory),
    stem: config.stem,
  );
}

/// Please see `loadExternalLibrary` for details
ExternalLibrary loadExternalLibraryRaw({
  Uri? nativeLibDirWhenNonPackaged,
  required String stem,
}) {
  // ref
  // * https://flutter.dev/docs/development/platform-integration/c-interop
  // * https://github.com/fzyzcjy/flutter_rust_bridge/pull/898

  ExternalLibrary tryAssumingNonPackaged(
      String name, ExternalLibrary Function(String debugInfo) fallback) {
    if (nativeLibDirWhenNonPackaged == null) {
      return fallback(
          '(without trying nativeLibDirWhenNonPackaged since it is null)');
    }

    final filePath = nativeLibDirWhenNonPackaged.resolve(name).toFilePath();
    if (!File(filePath).existsSync()) {
      return fallback('(after trying $filePath but it does not exist)');
    }

    return ExternalLibrary.open(filePath);
  }

  if (Platform.isAndroid) {
    return ExternalLibrary.open('lib$stem.so');
  }

  if (Platform.isIOS) {
    return ExternalLibrary.process();
  }

  if (Platform.isWindows) {
    final name = '$stem.dll';
    return tryAssumingNonPackaged(
        name, (debugInfo) => ExternalLibrary.open(name, debugInfo: debugInfo));
  }

  if (Platform.isMacOS) {
    final name = 'lib$stem.dylib';
    return tryAssumingNonPackaged(
        name, (debugInfo) => ExternalLibrary.process(debugInfo: debugInfo));
  }

  if (Platform.isLinux) {
    final name = 'lib$stem.so';
    return tryAssumingNonPackaged(
        name, (debugInfo) => ExternalLibrary.open(name, debugInfo: debugInfo));
  }

  // Feel free to PR to add support for more platforms! (e.g. I do not have a Fuchsia device, so cannot test that)
  throw Exception(
      'loadExternalLibrary failed: Unknown platform=${Platform.operatingSystem}');
}
