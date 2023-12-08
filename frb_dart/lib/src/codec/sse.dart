import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/misc/rust_vec_u8.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseCodec<S, E extends Object> extends BaseCodec<S, E> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final S Function(SseDeserializer deserializer) parseSuccessData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final E Function(SseDeserializer deserializer)? parseErrorData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const SseCodec({
    required this.parseSuccessData,
    required this.parseErrorData,
  });

  @override
  S decode(dynamic raw) {
    return TODO;
  }
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseSerializer {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  (ffi.Pointer<ffi.Uint8>, int) intoRaw() {
    return TODO;
  }
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseDeserializer {
  // TODO
}
