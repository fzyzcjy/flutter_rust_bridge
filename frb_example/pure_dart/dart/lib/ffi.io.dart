import 'dart:ffi';
import 'dart:io';

import 'bridge_generated.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(
    String path) {
  return FlutterRustBridgeExampleSingleBlockTestImpl(
    Platform.isMacOS || Platform.isIOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path),
  );
}
