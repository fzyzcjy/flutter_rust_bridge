import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(String path) {
  return FlutterRustBridgeExampleSingleBlockTestImpl(
    (Platform.isMacOS || Platform.isIOS) && !useOpenDylib ? DynamicLibrary.executable() : DynamicLibrary.open(path),
  );
}
