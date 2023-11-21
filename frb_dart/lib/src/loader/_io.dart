import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
ExternalLibrary loadExternalLibrary({
  // TODO give a better name
  Uri? attemptDirectory,
  required String stem,
}) {
  // ref
  // * https://flutter.dev/docs/development/platform-integration/c-interop
  // * https://github.com/fzyzcjy/flutter_rust_bridge/pull/898

  // TODO impl
  // TODO add logs for output, s.t. when error, can provide more info

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
