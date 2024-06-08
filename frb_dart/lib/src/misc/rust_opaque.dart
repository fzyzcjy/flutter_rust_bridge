import 'package:flutter_rust_bridge/src/rust_arc/_common.dart';
import 'package:meta/meta.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class RustOpaqueInterface {
  /// Dispose the underlying `Arc`.
  void dispose();

  /// Whether the underlying `Arc` is disposed.
  bool get isDisposed;
}

/// An opaque pointer to a native arbitrary Rust type.
/// TODO: link to the doc talking about "dispose"/GC semantics
abstract class RustOpaque implements RustOpaqueInterface {
  final RustArc _arc;

  /// If true, when sending this object to Rust, the ownership of this object
  /// will be released, mimicking the "move" semantics in Rust.
  bool? _move;

  set move(bool? move) => _move = move;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  RustOpaque.frbInternalDcoDecode(
      List<dynamic> wire, RustArcStaticData staticData)
      : this._fromRaw(
          ptr: wire[0],
          externalSizeOnNative: wire[1],
          staticData: staticData,
        );

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  RustOpaque.frbInternalSseDecode(
      BigInt ptr, int externalSizeOnNative, RustArcStaticData staticData)
      : this._fromRaw(
          ptr: ptr.toSigned(64).toInt(),
          externalSizeOnNative: externalSizeOnNative,
          staticData: staticData,
        );

  RustOpaque._fromRaw({
    required int ptr,
    required int externalSizeOnNative,
    required RustArcStaticData staticData,
  }) : _arc = RustArc.fromRaw(
          ptr: ptr,
          externalSizeOnNative: externalSizeOnNative,
          staticData: staticData,
        );

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  int frbInternalCstEncode({bool? move}) {
    assert(move == null || _move == null,
        'Cannot specify move semantics in two places');
    final effectiveMoveMode = move ?? _move ?? false;

    final target = effectiveMoveMode ? _arc : _arc.clone();
    return target.intoRaw();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  BigInt frbInternalSseEncode({bool? move}) =>
      BigInt.from(frbInternalCstEncode(move: move)).toUnsigned(64);

  /// Dispose the underlying `Arc`.
  @override
  void dispose() => _arc.dispose();

  /// Whether the underlying `Arc` is disposed.
  @override
  bool get isDisposed => _arc.isDisposed;
}
