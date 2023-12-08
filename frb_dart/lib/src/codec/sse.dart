import 'package:flutter_rust_bridge/src/codec/base.dart';

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
  // TODO
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class SseDeserializer {
  // TODO
}
