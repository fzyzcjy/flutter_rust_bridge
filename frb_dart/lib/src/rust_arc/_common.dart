import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:meta/meta.dart';

// ----- TODO this comment is originally at `dispose()`,
//  but since `dispose` is in `Droppable`, the comments no longer apply there
/// Call Rust destructors on the backing memory of this pointer.
///
/// This function should be run at least once during the lifetime of the
/// program, and can be run many times.
///
/// When passed into a Rust function, Rust enacts *shared ownership*,
/// if this pointer is shared with Rust when [dispose] is called,
/// ownership is fully transferred to Rust else this pointer is cleared.
// ----- TODO this is for `isDisposed`
/// Checks whether [dispose] has been called at any point during the lifetime
/// of this pointer. This does not guarantee that the backing memory has
/// actually been reclaimed.
// -----

/// The Rust `std::sync::Arc` on the Dart side.
// Note: Use `extends`, instead of making the `_Droppable` a field,
// in order to ensure the `ffi.Finalizable` works well.
abstract class RustArc extends Droppable {
  /// The pointer that `std::sync::Arc::into_raw` gives.
  ///
  /// In other words, it is very similar to `std::sync::Arc.ptr`,
  /// but only with a small constant offset.
  // We do this no-op override merely to provide documentations.
  @override
  @protected
  PlatformPointer? get resource => super.resource;

  /// Mimic `std::sync::Arc::from_raw`
  RustArc.fromRaw({required int ptr, required super.externalSizeOnNative})
      : super(ptrOrNullFromInt(ptr));

  /// See comments in [RustArcStaticData] for details.
  @override
  @protected
  RustArcStaticData get staticData;
}

/// Should have exactly *one* instance per *type*.
///
/// For example, all `std::sync::Arc<Apple>` objects should use one
/// `RustArcTypeInfo` object, while all `std::sync::Arc<Orange>`
/// objects should use another.
class RustArcStaticData extends DroppableStaticData {
  final RustArcIncrementStrongCountFnType _rustArcIncrementStrongCount;

  /// Constructs the data
  RustArcStaticData({
    /// Directly calls `std::sync::Arc::increment_strong_count(ptr)`
    required RustArcIncrementStrongCountFnType rustArcIncrementStrongCount,

    /// Directly calls `std::sync::Arc::decrement_strong_count(ptr)`
    required RustArcDecrementStrongCountFnType rustArcDecrementStrongCount,

    /// The function pointer to `rustArcDecrementStrongCount`
    required CrossPlatformFinalizerArg rustArcDecrementStrongCountPtr,
  })  : _rustArcIncrementStrongCount = rustArcIncrementStrongCount,
        super(
          releaseFn: rustArcDecrementStrongCount,
          releaseFnPtr: rustArcDecrementStrongCountPtr,
        );
}

/// The type of [RustArcStaticData._rustArcIncrementStrongCount]
typedef RustArcIncrementStrongCountFnType = void Function(PlatformPointer);

/// The type of [RustArcStaticData._rustArcDecrementStrongCount]
typedef RustArcDecrementStrongCountFnType = void Function(PlatformPointer);
