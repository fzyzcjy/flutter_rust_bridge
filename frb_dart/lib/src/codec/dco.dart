import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/platform_utils/platform_utils.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DcoCodec<S, E extends Object>
    extends BaseCodec<S, E, WireSyncRust2DartDco> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final S Function(dynamic) decodeSuccessData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final E Function(dynamic)? decodeErrorData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const DcoCodec({
    required this.decodeSuccessData,
    required this.decodeErrorData,
  });

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @override
  S decodeObject(dynamic raw) {
    // Required under dart2wasm: JS arrays arrive as `JSAny`, not Dart `List`. Mirrors sse.dart.
    raw = maybeDartify(raw);
    final rawList = raw as List<dynamic>;
    // `dartify()` returns JS numbers as Dart `double` on dart2wasm; coerce so the
    // typed `int action` parameter does not throw on the implicit cast.
    return _DcoSimpleDecoder(this, rawList).decode((rawList[0] as num).toInt());
  }

  @override
  S decodeWireSyncType(WireSyncRust2DartDco raw) =>
      decodeObject(wireSyncRust2DartDcoIntoDart(raw));

  @override
  void freeWireSyncRust2Dart(
    WireSyncRust2DartDco raw,
    GeneralizedFrbRustBinding generalizedFrbRustBinding,
  ) => generalizedFrbRustBinding.freeWireSyncRust2DartDco(raw);
}

class _DcoSimpleDecoder<S, E extends Object> extends SimpleDecoder<S, E> {
  final DcoCodec<S, E> codec;
  final List<dynamic> rawList;

  _DcoSimpleDecoder(this.codec, this.rawList);

  @override
  S decodeSuccess() {
    assert(rawList.length == 2);
    return codec.decodeSuccessData(rawList[1]);
  }

  @override
  E decodeError() {
    assert(rawList.length == 2);
    final decodeErrorData = codec.decodeErrorData;
    if (decodeErrorData == null) {
      throw Exception(
        'transformRust2DartMessage received error message, but no decodeErrorData to parse it. '
        'Raw data: $rawList',
      );
    }
    return decodeErrorData(rawList[1]);
  }

  @override
  Object decodePanic() {
    assert(rawList.length == 2);
    return dcoDecodePanicError(rawList[1]);
  }
}
