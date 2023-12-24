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

// This is tested via `flutter run` (or `flutter test`) under all platforms
// listed below in the CI, whose coverage is not gathered yet, so we temporarily
// disable it and re-enable later.
// coverage:ignore-start
/// Please see `loadExternalLibrary` for details
ExternalLibrary loadExternalLibraryRaw({
  Uri? nativeLibDirWhenNonPackaged,
  required String stem,
}) {
  // ref
  // * https://flutter.dev/docs/development/platform-integration/c-interop
  // * https://github.com/fzyzcjy/flutter_rust_bridge/pull/898
  // * https://github.com/flutter/flutter/blob/8b6277e63868c2029f1e2327879b7899be44fbe2/packages/flutter_tools/templates/plugin_ffi/lib/projectName.dart.tmpl#L47-L58

  ExternalLibrary tryAssumingNonPackaged(
      String name, ExternalLibrary Function(String debugInfo) fallback) {
    final effectiveNativeLibDir = Platform
            .environment['FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR']
            ?.toUriDirectory() ??
        nativeLibDirWhenNonPackaged;

    if (effectiveNativeLibDir == null) {
      return fallback(
          '(without trying effectiveNativeLibDir since it is null)');
    }

    final filePath = effectiveNativeLibDir.resolve(name).toFilePath();
    if (!File(filePath).existsSync()) {
      return fallback('(after trying $filePath but it does not exist)');
    }

    return ExternalLibrary.open(filePath);
  }

  if (Platform.isAndroid) {
    return ExternalLibrary.open('lib$stem.so');
  }

  if (Platform.isWindows) {
    final name = '$stem.dll';
    return tryAssumingNonPackaged(
        name, (debugInfo) => ExternalLibrary.open(name, debugInfo: debugInfo));
  }

  if (Platform.isIOS || Platform.isMacOS) {
    return tryAssumingNonPackaged(
      'lib$stem.dylib',
      (debugInfo) => _tryOpen(
        'rust_builder.framework/rust_builder',
        debugInfo,
        (debugInfo) =>
            ExternalLibrary.open('$stem.framework/$stem', debugInfo: debugInfo),
      ),
    );
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

ExternalLibrary _tryOpen(String name, String debugInfo,
    ExternalLibrary Function(String debugInfo) fallback) {
  try {
    return ExternalLibrary.open(name, debugInfo: debugInfo);
  } catch (e) {
    return fallback('$debugInfo (after trying $name but has error $e)');
  }
}

extension on String {
  Uri toUriDirectory() => Uri.directory(this);
}
// coverage:ignore-end
