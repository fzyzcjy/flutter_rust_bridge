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

// internal code
DynamicLibrary _openDylib() {
  // https://flutter.dev/docs/development/platform-integration/c-interop
  if (Platform.isAndroid || Platform.isLinux) return DynamicLibrary.open('libvision_utils_rs.so');
  if (Platform.isIOS) return DynamicLibrary.process();

  if (Platform.isWindows) {
    final dllAbsolutePath = path.join(path.dirname(Platform.resolvedExecutable), 'vision_utils_rs.dll');
    Log.d(_kTag, 'openDylib dllAbsolutePath=$dllAbsolutePath', self: null);
    return DynamicLibrary.open(dllAbsolutePath);
  }

  if (Platform.isMacOS) {
    // e.g. pure dart
    try {
      Log.d(_kTag, 'openDylib try .dylib file', self: null);
      return DynamicLibrary.open('libvision_utils_rs.dylib');
    } catch (e) {
      Log.i(_kTag, 'error when open e=$e', self: null);
    }

    try {
      final libVisionUtilsPath =
          path.join(repoBaseDir!, 'frontend/vision_utils/rust/target/release/libvision_utils_rs.dylib');
      Log.d(_kTag, 'openDylib try using absolute path libVisionUtilsPath=$libVisionUtilsPath', self: null);

      if (!File(libVisionUtilsPath).existsSync()) {
        throw Exception(
            'libVisionUtilsPath=$libVisionUtilsPath does not exist - you may need to compile Rust code first');
      }

      return DynamicLibrary.open(libVisionUtilsPath);
    } catch (e) {
      Log.i(_kTag, 'error when open e=$e', self: null);
    }

    // print('=================== NOTE ===================\n'
    //     'You may want to:\n'
    //     '1. run `bundle exec fastlane build_macos profile:release` in vision_utils/rust to build dylib file\n'
    //     '2. set `DYLD_LIBRARY_PATH=...` to let Dart find the dylib\n'
    //     'Remark: For more information, see https://github.com/fzyzcjy/yplusplus/issues/1610\n'
    //     '=============================================');

    Log.i(_kTag, 'openDylib fallback to `process` mode. This is expected behavior when running with Flutter.',
        self: null);
    return DynamicLibrary.process();
  }

  throw Exception(
      'Do not know how to find vision_utils rust library, you may want to modify the library finding code in vu_core.dart, platform=${Platform.operatingSystem}');
}
