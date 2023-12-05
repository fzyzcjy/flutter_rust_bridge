import 'package:meta/meta.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
Object api2wireDartOpaque(Object raw) {
  // TODO rm the port thing
  return [raw, portManager.dartOpaqueDropPort];
}
