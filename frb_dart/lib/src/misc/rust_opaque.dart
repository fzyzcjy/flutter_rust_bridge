import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_common.dart';
import 'package:meta/meta.dart';

/// An opaque pointer to a native arbitrary Rust type.
/// TODO: link to the doc talking about "dispose"/GC semantics
abstract class RustOpaque {
  final RustArc _arc;

  /// If true, when sending this object to Rust, the ownership of this object
  /// will be released, mimicking the "move" semantics in Rust.
  bool? _move;

  set move(bool? move) => _move = move;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  RustOpaque.dcoDecode(List<dynamic> wire, RustArcStaticData staticData)
      : this._fromRaw(
          ptr: wire[0],
          externalSizeOnNative: wire[1],
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
  PlatformPointer cstEncode({bool? move}) {
    assert(move == null || _move == null,
        'Cannot specify move semantics in two places');
    final effectiveMoveMode = move ?? _move ?? false;

    final target = effectiveMoveMode ? _arc : _arc.clone();
    return target.intoRaw();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  int sseEncode({bool? move}) =>
      PlatformPointerUtil.ptrToInt(cstEncode(move: move));

  /// Dispose the underlying `Arc`.
  void dispose() => _arc.dispose();

  /// Whether the underlying `Arc` is disposed.
  bool get isDisposed => _arc.isDisposed;
}
