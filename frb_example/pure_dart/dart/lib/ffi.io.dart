// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/utils.io.dart';

import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(String path) {
  return FlutterRustBridgeExampleSingleBlockTestImpl(open(
      maybeUserDefinedKind: DylibSourceKind.fromEnvironment(dylibSource),
      path: path,
      ctx: LanguageExecutionContext.dart));
}
