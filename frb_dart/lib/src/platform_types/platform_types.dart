import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

export '_io.dart' if (dart.library.js_interop) '_web.dart';

/// {@macro flutter_rust_bridge.internal}
@internal
abstract class BaseExternalLibrary {
  final String debugInfo;

  const BaseExternalLibrary({required this.debugInfo});
}
