import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseApi {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseApi();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void frbInitializeRust({
    required NativePortType dartOpaqueDropPort,
    required NativePortType dartFnInvokePort,
  });
}
