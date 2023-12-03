import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_common.dart';
import 'package:meta/meta.dart';

// TODO this should be Finalizable as well?
// TODO comments
/// An opaque pointer to a native Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
abstract class RustOpaque {
  final RustArc _arc;

  /// Displays the need to release ownership when sending to rust.
  bool _move = false;

  set move(bool move) => _move = move;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  RustOpaque.fromWire(List<dynamic> wire)
      : _arc = RustArc.fromRaw(
          ptr: wire[0],
          externalSizeOnNative: wire[1],
          staticData: staticData,
        );

  /// Increments inner reference counter and returns pointer to the underlying
  /// Rust object.
  ///
  /// Throws a [StateError] if called after [dispose].
  ///
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  PlatformPointer shareOrMove() {
    if (!isDisposed()) {
      var ptr = shareFn(_ptr);
      if (_move) {
        dispose();
      }
      return ptr;
    } else {
      return PlatformPointerUtil.nullPtr();
    }
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  RustArcStaticData get staticData;
}
