import 'package:flutter_rust_bridge/src/droppable/_io.dart'
    if (dart.library.html) '_web.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:meta/meta.dart';

/// Encapsulates the [resource] release logic.
///
/// In Rust, it is simple to release some resource: Just implement `Drop` trait.
/// However, there are two possible chances to release resource in Dart:
/// 1. When the object is garbage collected, the Dart finalizer will call a callback you choose.
/// 2. When the user explicitly calls `dispose()` function, you can do releasing job.
///
/// But we want to release the [resource] *once and exactly once*.
/// That's what this class does.
///
/// You just implement the `releaseFn` (and `releaseFnPtr`), and this class
/// ensures it is called exactly once.
abstract class Droppable implements DroppableBase {
  /// {@macro flutter_rust_bridge.internal}
  @protected
  PlatformPointer? get resource => _resource;
  PlatformPointer? _resource;

  /// {@macro flutter_rust_bridge.internal}
  Droppable(this._resource, {required int size}) {
    if (_resource != null) {
      // Note: The finalizer attaches to the `_ptr` at *current* time,
      // thus even if we assign `RustArc._ptr = something-new`, this finalizer
      // attachment will not be changed.
      staticData._finalizer.attachCrossPlatform(this, _resource!,
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
      final resource = _resource!;
      _resource = null;
      assert(isDisposed());

      staticData._finalizer.detach(this);
      staticData._releaseFn(resource);
    }
  }

  // TODO comments
  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has
  /// actually been reclaimed.
  bool isDisposed() => _resource == null;

  /// See comments in [DroppableStaticData].
  @protected
  DroppableStaticData get staticData;
}

/// This data SHOULD be held as *static variable*, and only one object
/// for all instances of the type.
///
/// This is because the [_finalizer] should be static.
class DroppableStaticData {
  // TODO rename type etc
  final void Function(PlatformPointer) _releaseFn;

  /// The function pointer for [_rustArcDecrementStrongCount] on native platform.
  final CrossPlatformFinalizerArg _releaseFnPtr;

  late final _finalizer = CrossPlatformFinalizer(_releaseFnPtr);

  /// Constructs the data
  DroppableStaticData({
    required void Function(PlatformPointer) releaseFn,
    required CrossPlatformFinalizerArg releaseFnPtr,
  })  : _releaseFn = releaseFn,
        _releaseFnPtr = releaseFnPtr;
}
