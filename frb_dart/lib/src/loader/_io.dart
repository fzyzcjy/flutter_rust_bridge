import 'dart:async';
import 'dart:io';

import 'package:flutter_rust_bridge/src/loader/_common.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';
import 'package:meta/meta.dart';

/// Load the [ExternalLibrary], with the following cases in mind:
/// 1. When `flutter run`, or when a real app is bundled.
/// 2. When running Flutter widget tests.
/// 3. When `dart test`, `dart run`, `dart compile exe`, etc.
FutureOr<ExternalLibrary> loadExternalLibrary(
  ExternalLibraryLoaderConfig config,
) async {
  final ioDirectory = config.ioDirectory;
  return loadExternalLibraryRaw(
    nativeLibDirWhenNonPackaged: ioDirectory == null
        ? null
        : Directory.current.uri.resolve(ioDirectory),
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
    String name,
    ExternalLibrary Function(String debugInfo) fallback,
  ) {
    final effectiveNativeLibDir =
        Platform.environment['FRB_DART_LOAD_EXTERNAL_LIBRARY_NATIVE_LIB_DIR']
            ?.toUriDirectory() ??
        nativeLibDirWhenNonPackaged;

    if (effectiveNativeLibDir == null) {
      return fallback(
        '(without trying effectiveNativeLibDir since it is null)',
      );
    }

    final filePath = effectiveNativeLibDir.resolve(name).toFilePath();
    if (!File(filePath).existsSync()) {
      return fallback('(after trying $filePath but it does not exist)');
    }

    return ExternalLibrary.open(filePath);
  }

  if (Platform.isAndroid || Platform.operatingSystem == 'ohos') {
    return ExternalLibrary.open('lib$stem.so');
  }

  if (Platform.isWindows) {
    final name = '$stem.dll';
    return tryAssumingNonPackaged(
      name,
      (debugInfo) => ExternalLibrary.open(name, debugInfo: debugInfo),
    );
  }

  if (Platform.isIOS || Platform.isMacOS) {
    return tryAssumingNonPackaged(
      'lib$stem.dylib',
      (debugInfo) => loadDarwinPackagedExternalLibrary(
        stem: stem,
        debugInfo: debugInfo,
        open: (name, debugInfo) =>
            ExternalLibrary.open(name, debugInfo: debugInfo),
        process: (debugInfo) => ExternalLibrary.process(
          iKnowHowToUseIt: true,
          debugInfo: debugInfo,
        ),
      ),
    );
  }

  if (Platform.isLinux) {
    final name = 'lib$stem.so';
    return tryAssumingNonPackaged(
      name,
      (debugInfo) => ExternalLibrary.open(name, debugInfo: debugInfo),
    );
  }

  // Feel free to PR to add support for more platforms! (e.g. I do not have a Fuchsia device, so cannot test that)
  throw Exception(
    'loadExternalLibrary failed: Unknown platform=${Platform.operatingSystem}',
  );
}

/// Load a packaged Darwin library.
///
/// Falls back to process lookup for CocoaPods static linkage, where the Rust
/// symbols are linked into the app image instead of an embedded framework.
@visibleForTesting
T loadDarwinPackagedExternalLibrary<T>({
  required String stem,
  required String debugInfo,
  required T Function(String name, String debugInfo) open,
  required T Function(String debugInfo) process,
}) {
  return _tryOpenLibrary(
    'rust_builder.framework/rust_builder',
    debugInfo,
    open,
    (debugInfo) => _tryOpenLibrary(
      '$stem.framework/$stem',
      debugInfo,
      open,
      (debugInfo) => process('$debugInfo (after falling back to process())'),
    ),
  );
}

T _tryOpenLibrary<T>(
  String name,
  String debugInfo,
  T Function(String name, String debugInfo) open,
  T Function(String debugInfo) fallback,
) {
  try {
    return open(name, debugInfo);
  } catch (e) {
    return fallback('$debugInfo (after trying $name but has error $e)');
  }
}

extension on String {
  Uri toUriDirectory() => Uri.directory(this);
}

// coverage:ignore-end
