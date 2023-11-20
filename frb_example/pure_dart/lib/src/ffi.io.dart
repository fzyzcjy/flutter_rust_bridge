import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_definitions.dart';
import 'bridge_generated.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(
  String path,
) =>
    _api = FlutterRustBridgeExampleSingleBlockTestImpl(loadLibForDart(path));

FlutterRustBridgeExampleSingleBlockTest? _api;
FlutterRustBridgeExampleSingleBlockTest get api => _api ?? (throw Exception('API not yet initialized'));
