import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(String path) {
  return FlutterRustBridgeExampleSingleBlockTestImpl(open(
      maybeUserDefinedKind: DylibSourceKind.fromEnvironment(dylibSource),
      path: path,
      ctx: LanguageExecutionContext.dart));
}
