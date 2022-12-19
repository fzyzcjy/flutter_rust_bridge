import 'dart:io';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.io.dart';

const base = 'flutter_rust_bridge_example';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = loadDylib(path);
late final api = FlutterRustBridgeExampleImpl(dylib);
