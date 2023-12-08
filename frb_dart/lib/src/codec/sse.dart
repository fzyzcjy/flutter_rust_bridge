import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/write_buffer.dart';

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
  final WriteBuffer _buffer;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  SseSerializer(GeneralizedFrbRustBinding binding)
      : _buffer = WriteBuffer(binding: binding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  WriteBufferRaw intoRaw() => _buffer.intoRaw();
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseDeserializer {
  // TODO
}
