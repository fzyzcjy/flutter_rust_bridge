import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_generated.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(String path) {
  return FlutterRustBridgeExampleSingleBlockTestImpl(
    (Platform.isMacOS || Platform.isIOS) && !readBoolEnv('SILICON')
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path),
  );
}
