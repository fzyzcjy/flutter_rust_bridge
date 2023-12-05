import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseApi {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseApi();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void frbInitializeRust(FrbInitializeRustData data);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class FrbInitializeRustData {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final NativePortType dartOpaqueDropPort;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final NativePortType dartFnInvokePort;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const FrbInitializeRustData({
    required this.dartOpaqueDropPort,
    required this.dartFnInvokePort,
  });
}
