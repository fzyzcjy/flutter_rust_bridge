import 'package:meta/meta.dart';
import 'ffi/io.dart' if (dart.library.html) 'ffi/web.dart';

export 'ffi/stub.dart'
    if (dart.library.io) 'ffi/io.dart'
    if (dart.library.html) 'ffi/web.dart';

typedef DropFnType = void Function(PlatformPointer);
typedef ShareFnType = PlatformPointer Function(PlatformPointer);

/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
abstract class FrbOpaque extends FrbOpaqueImpl {
  /// Pointer to this opaque Rust type.
  PlatformPointer _ptr;

  /// A native finalizer rust opaque type.
  /// Is static for each frb api class instance.
  OpaqueTypeFinalizer get staticFinalizer;

  /// Rust type specific drop function.
  ///
  /// This function should never be called manually.
  DropFnType get dropFn;

  /// Rust type specific share function.
  ///
  /// This function should never be called manually.
  ShareFnType get shareFn;

  /// This constructor should never be called manually.
  @internal
  FrbOpaque.unsafe(int ptr, int size) : _ptr = FrbOpaqueImpl.initPtr(ptr) {
    assert(ptr > 0);
    FrbOpaqueImpl.finalizerAttach(this, _ptr, size, staticFinalizer);
  }

  /// Call Rust destructors on the backing memory of this pointer.
  ///
  /// This function should be run at least once during the lifetime of the
  /// program, and can be run many times.
  ///
  /// When passed into a Rust function, Rust enacts *shared ownership*,
  /// if this pointer is shared with Rust when [dispose] is called,
  /// ownership is fully transferred to Rust else this pointer is cleared.
  void dispose() {
    if (!isStale()) {
      var ptr = _ptr;
      _ptr = FrbOpaqueImpl.nullPtr();

      staticFinalizer.detach(this);
      dropFn(ptr);
    }
  }

  /// Increments inner reference counter and returns pointer to the underlying
  /// Rust object.
  ///
  /// Throws a [StateError] if called after [dispose].
  @internal
  PlatformPointer share() {
    if (!isStale()) {
      return shareFn(_ptr);
    } else {
      throw StateError('Use after dispose.');
    }
  }

  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has
  /// actually been reclaimed.
  bool isStale() => FrbOpaqueImpl.isStalePtr(_ptr);
}
