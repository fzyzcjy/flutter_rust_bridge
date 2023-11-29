import 'package:meta/meta.dart';

export '_io.dart' if (dart.library.html) '_web.dart';

/// {@macro flutter_rust_bridge.internal}
@internal
abstract class BaseExternalLibrary {
  final String debugInfo;

  const BaseExternalLibrary({required this.debugInfo});
}
