import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DropPortManager {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final GeneralizedFrbRustBinding _generalizedFrbRustBinding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  DropPortManager(this._generalizedFrbRustBinding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  NativePortType get dropPort => _dropPort.sendPort.nativePort;
  late final _dropPort = _initDropPort();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  ReceivePort _initDropPort() {
    final port = broadcastPort(DropIdPortGenerator.create());
    port.listen((message) {
      _generalizedFrbRustBinding.dropDartObject(message);
    });
    return port;
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dispose() {
    _dropPort.close();
  }
}
