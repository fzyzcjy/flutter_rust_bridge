import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_generated.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(String path) =>
    FlutterRustBridgeExampleSingleBlockTestImpl(
      loadDylib(path),
    );
