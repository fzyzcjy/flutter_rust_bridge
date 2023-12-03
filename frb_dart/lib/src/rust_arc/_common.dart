import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_io.dart'
    if (dart.library.html) '_web.dart';
import 'package:meta/meta.dart';

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
abstract class _Droppable<T extends Object> implements DroppableBase {
  T? get _resource => __resource;
  T? __resource;

  _Droppable(this.__resource, {required int size}) {
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
  _DroppablePerTypeData get perTypeData;
}

class _DroppablePerTypeData<T> {
  // TODO rename type etc
  final void Function(T) _releaseFn;

  /// The function pointer for [_rustArcDecrementStrongCount] on native platform.
  final ArcTypeFinalizerArg _releaseFnPtr;

  late final _finalizer = ArcTypeFinalizer(_releaseFnPtr);

  /// Constructs the data
  _DroppablePerTypeData({
    required void Function(T) releaseFn,
    required ArcTypeFinalizerArg releaseFnPtr,
  })  : _releaseFn = releaseFn,
        _releaseFnPtr = releaseFnPtr;
}

/// The Rust `std::sync::Arc` on the Dart side.
// Note: Use `extends`, instead of making the `_Droppable` a field,
// in order to ensure the `ffi.Finalizable` works well.
abstract class RustArc extends _Droppable<PlatformPointer> {
  /// Either the pointer that `std::sync::Arc::into_raw` gives,
  /// or a null pointer.
  ///
  /// In other words, when non-null, it is very similar to `std::sync::Arc.ptr`,
  /// but only with a small constant offset.
  // We do this no-op override merely to provide documentations.
  @override
  PlatformPointer? get _resource => super._resource;

  /// Mimic `std::sync::Arc::from_raw`
  RustArc.fromRaw({required int ptr, required super.size})
      : super(ptrOrNullFromInt(ptr));

  /// See comments in [RustArcPerTypeData] for details.
  @protected
  RustArcPerTypeData get perTypeData;
}

/// Should have exactly *one* instance per *type*.
///
/// For example, all `std::sync::Arc<Apple>` objects should use one
/// `RustArcTypeInfo` object, while all `std::sync::Arc<Orange>`
/// objects should use another.
class RustArcPerTypeData {
  // TODO rename: shareFn -> rust_arc_increment_strong_count
  // TODO comments
  /// Directly calls `std::sync::Arc::increment_strong_count(ptr)`
  final RustArcIncrementStrongCountFnType _rustArcIncrementStrongCount;

  // TODO rename: releaseFn -> rust_arc_decrement_strong_count
  // TODO comments
  /// Directly calls `std::sync::Arc::decrement_strong_count(ptr)`
  final RustArcDecrementStrongCountFnType _rustArcDecrementStrongCount;

  /// The function pointer for [_rustArcDecrementStrongCount] on native platform.
  final ArcTypeFinalizerArg _rustArcDecrementStrongCountPtr;

  late final _finalizerByArcDecrCount =
      ArcTypeFinalizer(_rustArcDecrementStrongCountPtr);

  /// Constructs the data
  RustArcPerTypeData({
    required RustArcIncrementStrongCountFnType rustArcIncrementStrongCount,
    required RustArcDecrementStrongCountFnType rustArcDecrementStrongCount,
    required ArcTypeFinalizerArg rustArcDecrementStrongCountPtr,
  })  : _rustArcDecrementStrongCount = rustArcDecrementStrongCount,
        _rustArcIncrementStrongCount = rustArcIncrementStrongCount,
        _rustArcDecrementStrongCountPtr = rustArcDecrementStrongCountPtr;
}

/// The type of [RustArcPerTypeData._rustArcIncrementStrongCount]
typedef RustArcIncrementStrongCountFnType = void Function(PlatformPointer);

/// The type of [RustArcPerTypeData._rustArcDecrementStrongCount]
typedef RustArcDecrementStrongCountFnType = void Function(PlatformPointer);
