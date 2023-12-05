import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
wire_DartOpaque api2wireDartOpaque(
    Object raw, GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  final ptr = wire.new_DartOpaque();
  _api_fill_to_wire_DartOpaque(raw, ptr);
  return ptr;
}

// TODO rm these
void _api_fill_to_wire_DartOpaque(Object apiObj, wire_DartOpaque wireObj) {
  wireObj.handle = generalizedFrbRustBinding.newDartOpaque(apiObj);
  wireObj.port = portManager.dartOpaqueDropPort;
}

// TODO rm these
final class wire_DartOpaque extends ffi.Struct {
  @ffi.Int64()
  external int port;

  @ffi.UintPtr()
  external int handle;
}
