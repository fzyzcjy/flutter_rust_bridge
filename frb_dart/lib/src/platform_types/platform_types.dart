import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

export '_io.dart' if (dart.library.html) '_web.dart';

/// {@macro flutter_rust_bridge.internal}
@internal
abstract class BaseExternalLibrary {
  final String debugInfo;

  const BaseExternalLibrary({required this.debugInfo});
}

/// {@macro flutter_rust_bridge.internal}
PlatformPointer? ptrOrNullFromInt(int ptr) {
  final p = PlatformPointerUtil.ptrFromInt(ptr);
  return PlatformPointerUtil.isNullPtr(p) ? null : p;
}
