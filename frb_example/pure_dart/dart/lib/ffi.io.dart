// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/utils.io.dart';

import 'bridge_generated.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(String path) =>
    FlutterRustBridgeExampleSingleBlockTestImpl(
      loadDylib(path),
    );
