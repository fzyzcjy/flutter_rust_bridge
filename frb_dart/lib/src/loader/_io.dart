import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
ExternalLibrary loadExternalLibrary({
  required String name,
}) {
  // also see https://github.com/fzyzcjy/flutter_rust_bridge/pull/898

  if (Platform.isIOS) {
    return DynamicLibrary.process();
  }

  if (Platform.isMacOS) {
    if (Abi.current() == Abi.macosX64) {
      return DynamicLibrary.executable();
    }
  }

  return DynamicLibrary.open(path);
}
