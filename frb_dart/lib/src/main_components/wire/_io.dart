import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

export '_io.dart' if (dart.library.html) '_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final ExternalLibrary externalLibrary;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseWire({required this.externalLibrary});
}
