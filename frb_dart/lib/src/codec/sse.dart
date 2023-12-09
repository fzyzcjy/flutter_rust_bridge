import 'dart:ffi' as ffi;
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/read_buffer.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/write_buffer.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseCodec<S, E extends Object> extends BaseCodec<S, E, WireSyncReturnSse> {
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
  S decodeObject(dynamic raw) => _decode(raw as Uint8List);

  @override
  S decodeWireSyncType(WireSyncReturnSse raw) =>
      _decode(raw.ptr.asTypedList(raw.len));

  S _decode(Uint8List bytes) {
    final deserializer = SseDeserializer(bytes.buffer.asByteData());
    final action = deserializer.buffer.getUint8();
    final ans = SimpleDecoder().decode(action);
    assert(!deserializer.buffer.hasRemaining);
    return ans;
  }

  @override
  void freeWireSyncReturn(WireSyncReturnSse raw,
          GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
      generalizedFrbRustBinding.freeWireSyncReturnSse(raw);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseSerializer {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final WriteBuffer buffer;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  SseSerializer(GeneralizedFrbRustBinding binding)
      : buffer = WriteBuffer(binding: binding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  WriteBufferRaw intoRaw() => buffer.intoRaw();
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseDeserializer {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final ReadBuffer buffer;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  SseDeserializer(ByteData data) : buffer = ReadBuffer(data);
}
