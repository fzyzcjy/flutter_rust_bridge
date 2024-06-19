import 'package:flutter_rust_bridge/src/droppable/_io.dart'
    if (dart.library.js_interop) '_web.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:meta/meta.dart';

/// Encapsulates the [internalResource] release logic.
///
/// In Rust, it is simple to release some resource: Just implement `Drop` trait.
/// However, there are two possible chances to release resource in Dart:
/// 1. When the object is garbage collected, the Dart finalizer will call a callback you choose.
/// 2. When the user explicitly calls `dispose()` function, you can do releasing job.
///
/// But we want to release the [internalResource] *once and exactly once*.
/// That's what this class does.
///
/// You just implement the `releaseFn` (and `releaseFnPtr`), and this class
/// ensures it is called exactly once.
abstract class Droppable implements DroppableBase {
  /// NEVER read it directly outside subclasses,
  /// otherwise all encapsulation breaks down.
  @protected
  PlatformPointer dangerousReadInternalPtr() =>
      _ptr ?? (throw DroppableDisposedException('$runtimeType'));

  /// null := already disposed
  PlatformPointer? _ptr;

  /// {@macro flutter_rust_bridge.internal}
  @protected
  final int externalSizeOnNative;

  /// {@macro flutter_rust_bridge.internal}
  Droppable({required PlatformPointer ptr, required this.externalSizeOnNative})
      : assert(!PlatformPointerUtil.isNullPtr(ptr)),
        _ptr = ptr {
    // Note: The finalizer attaches to the `_ptr` at *current* time,
    // thus even if we assign `RustArc._ptr = something-new`, this finalizer
    // attachment will not be changed.
    staticData._finalizer.attachCrossPlatform(this, _ptr!,
        detach: this, externalSizeOnNative: externalSizeOnNative);
  }

  /// Disposes the resource.
  void dispose() {
    if (!isDisposed) {
      // Forget it before calling `releaseFn`.
      // If the contrary, when something bad happens in between,
      // the data will be released at least twice - one by calling releaseFn,
      // another by future call to `dispose` or the auto invocation of `finalizer`.
      final ptr = _ptr!;
      forget();
      staticData._releaseFn(ptr);
    }
  }

  /// Mimic `std::mem::forget`
  void forget() {
    if (!isDisposed) {
      _ptr = null;
      staticData._finalizer.detach(this);
    }
  }

  /// Check whether the resource is disposed.
  bool get isDisposed => _ptr == null;

  /// See comments in [DroppableStaticData] for requirements.
  @protected
  DroppableStaticData get staticData;
}

/// This data SHOULD be held as *static variable*, and only one object
/// for all instances of the type.
///
/// This is because the [_finalizer] should be static.
class DroppableStaticData {
  /// The function to release the resource
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

/// {@macro flutter_rust_bridge.internal}
class DroppableDisposedException implements FrbException {
  /// {@macro flutter_rust_bridge.internal}
  final String name;

  /// {@macro flutter_rust_bridge.internal}
  const DroppableDisposedException(this.name);

  @override
  String toString() => 'DroppableDisposedException: '
      'Try to use `$name` after it has been disposed';
}
