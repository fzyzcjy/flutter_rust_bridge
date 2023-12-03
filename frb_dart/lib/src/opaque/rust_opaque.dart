import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:meta/meta.dart';

// TODO this should be Finalizable as well?
// TODO comments
/// An opaque pointer to a native Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
abstract class RustOpaque extends RustOpaqueBase {
  /// Displays the need to release ownership when sending to rust.
  bool _move = false;

  set move(bool move) => _move = move;

  /// This constructor should never be called manually.
  @internal
  RustOpaque.fromWire(dynamic wire) : this._raw(wire[0], wire[1]);

  /// Increments inner reference counter and returns pointer to the underlying
  /// Rust object.
  ///
  /// Throws a [StateError] if called after [dispose].
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
}
