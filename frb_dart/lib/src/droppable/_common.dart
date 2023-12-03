import 'package:flutter_rust_bridge/src/droppable/_io.dart'
    if (dart.library.html) '_web.dart';

/// Encapsulates the [_resource] release logic.
///
/// In Rust, it is simple to release some resource: Just implement `Drop` trait.
/// However, there are two possible chances to release resource in Dart:
/// 1. When the object is garbage collected, the Dart finalizer will call a callback you choose.
/// 2. When the user explicitly calls `dispose()` function, you can do releasing job.
///
/// But we want to release the [_resource] *once and exactly once*.
/// That's what this class does.
///
/// You just implement the `releaseFn` (and `releaseFnPtr`), and this class
/// ensures it is called exactly once.
abstract class Droppable<T extends Object> implements DroppableBase {
  T? get _resource => __resource;
  T? __resource;

  /// {@macro flutter_rust_bridge.internal}
  Droppable(this.__resource, {required int size}) {
    if (__resource != null) {
      // Note: The finalizer attaches to the `_ptr` at *current* time,
      // thus even if we assign `RustArc._ptr = something-new`, this finalizer
      // attachment will not be changed.
      perTypeData._finalizer.attachCrossPlatform(this, __resource,
          detach: this, externalSizeOnNative: size);
    }
  }

  // TODO comments
  /// Call Rust destructors on the backing memory of this pointer.
  ///
  /// This function should be run at least once during the lifetime of the
  /// program, and can be run many times.
  ///
  /// When passed into a Rust function, Rust enacts *shared ownership*,
  /// if this pointer is shared with Rust when [dispose] is called,
  /// ownership is fully transferred to Rust else this pointer is cleared.
  void dispose() {
    if (!isDisposed()) {
      final resource = __resource;
      __resource = null;
      assert(isDisposed());

      perTypeData._finalizer.detach(this);
      perTypeData._releaseFn(resource);
    }
  }

  // TODO comments
  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has
  /// actually been reclaimed.
  bool isDisposed() => __resource == null;

  // TODO mention this should be static
  DroppablePerTypeData get perTypeData;
}

// TODO wrong, should say "static data", since *finalizer* should be static and not GCed itself
/// {@macro flutter_rust_bridge.internal}
class DroppablePerTypeData<T> {
  // TODO rename type etc
  final void Function(T) _releaseFn;

  /// The function pointer for [_rustArcDecrementStrongCount] on native platform.
  final CrossPlatformFinalizerArg _releaseFnPtr;

  late final _finalizer = CrossPlatformFinalizer(_releaseFnPtr);

  /// Constructs the data
  DroppablePerTypeData({
    required void Function(T) releaseFn,
    required CrossPlatformFinalizerArg releaseFnPtr,
  })  : _releaseFn = releaseFn,
        _releaseFnPtr = releaseFnPtr;
}
