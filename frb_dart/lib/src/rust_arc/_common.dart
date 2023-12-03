import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_io.dart'
    if (dart.library.html) '_web.dart';
import 'package:meta/meta.dart';

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
class RustArcPerTypeData extends _DroppablePerTypeData<PlatformPointer> {
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
