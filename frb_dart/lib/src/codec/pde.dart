import 'package:flutter_rust_bridge/src/codec/sse.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
void pdeCallFfi(
  GeneralizedFrbRustBinding generalizedFrbRustBinding,
  SseSerializer serializer, {
  required int funcId,
  required NativePortType port,
}) {
  final raw = serializer.intoRaw();
  generalizedFrbRustBinding.pdeFfiDispatcher(
    funcId: funcId,
    port: port,
    ptr: raw.ptr,
    rustVecLen: raw.rustVecLen,
    dataLen: raw.dataLen,
  );
}
