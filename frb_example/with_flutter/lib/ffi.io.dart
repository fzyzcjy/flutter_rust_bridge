import 'dart:ffi';
import 'dart:io';

import 'bridge_generated.io.dart';

const base = 'flutter_rust_bridge_example';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
late final api = FlutterRustBridgeExampleImpl(dylib);
