import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/read_buffer.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/write_buffer.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseCodec<S, E extends Object> extends BaseCodec<S, E, WireSyncReturnSse> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final S Function(SseDeserializer deserializer) decodeSuccessData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final E Function(SseDeserializer deserializer)? decodeErrorData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const SseCodec({
    required this.decodeSuccessData,
    required this.decodeErrorData,
  });

  @override
  S decodeObject(dynamic raw) => _decode(raw as Uint8List);

  @override
  S decodeWireSyncType(WireSyncReturnSse raw) =>
      _decode(raw.ptr.asTypedList(raw.len));

  S _decode(Uint8List bytes) {
    final deserializer = SseDeserializer(bytes.buffer.asByteData());
    final action = deserializer.buffer.getUint8();
    final ans = _SseSimpleDecoder(this, deserializer).decode(action);
    assert(!deserializer.buffer.hasRemaining);
    return ans;
  }

  @override
  void freeWireSyncReturn(WireSyncReturnSse raw,
          GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
      generalizedFrbRustBinding.freeWireSyncReturnSse(raw);
}

class _SseSimpleDecoder<S, E extends Object> extends SimpleDecoder<S, E> {
  final SseCodec<S, E> codec;
  final SseDeserializer deserializer;

  _SseSimpleDecoder(this.codec, this.deserializer);

  @override
  S decodeSuccess() => codec.decodeSuccessData(deserializer);

  @override
  E decodeError() {
    final decodeErrorData = codec.decodeErrorData;
    if (decodeErrorData == null) {
      throw Exception(
          'transformRust2DartMessage received error message, but no decodeErrorData to parse it.');
    }
    return decodeErrorData(deserializer);
  }

  @override
  Object decodePanic() => sseDecodePanicError(deserializer);
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
