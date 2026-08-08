import 'package:flutter_rust_bridge/src/codec/sse.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class PdeCodec<S, E extends Object> extends SseCodec<S, E> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const PdeCodec({
    required S Function(SseDeserializer deserializer) decodeSuccessDataSse,
    required E Function(SseDeserializer deserializer)? decodeErrorDataSse,
    required S Function(dynamic) decodeSuccessDataDco,
    required E Function(dynamic)? decodeErrorDataDco,
  }) : super(
         decodeSuccessData: decodeSuccessDataSse,
         decodeErrorData: decodeErrorDataSse,
       );
}
