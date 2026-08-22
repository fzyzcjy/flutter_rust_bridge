import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// The Rust `std::sync::Arc` on the Dart side.
// Note: Use `extends`, instead of making the `_Droppable` a field,
// in order to ensure the `ffi.Finalizable` works well.
class RustArc<T> extends Droppable {
  /// The pointer that `std::sync::Arc::into_raw` gives.
  ///
  /// In other words, it is very similar to `std::sync::Arc.ptr`,
  /// but only with a small constant offset.
  int get _ptr =>
      PlatformPointerUtil.ptrToInt(super.dangerousReadInternalPtr());

  /// See comments in [RustArcStaticData] for details.
  final RustArcStaticData<T> _staticData;

  /// Mimic `std::sync::Arc::from_raw`
  RustArc.fromRaw({
    required int ptr,
    required super.externalSizeOnNative,
    required RustArcStaticData<T> staticData,
  }) : assert(ptr != 0),
       _staticData = staticData,
       super(ptr: PlatformPointerUtil.ptrFromInt(ptr));

  /// Mimic `std::sync::Arc::clone`
  RustArc<T> clone() {
    final ptr = _ptr;

    _staticData._rustArcIncrementStrongCount(
      PlatformPointerUtil.ptrFromInt(ptr),
    );

    return RustArc.fromRaw(
      ptr: ptr,
      externalSizeOnNative: externalSizeOnNative,
      staticData: _staticData,
    );
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  RustArcTransfer<T> prepareTransfer({required bool move}) => RustArcTransfer._(
    arc: move ? this : clone(),
    source: this,
    rollback: !move,
  );

  /// Mimic `std::sync::Arc::into_raw`
  // Almost 1:1 implementation to `std::sync::Arc::into_raw` impl.
  int intoRaw() {
    final ptr = _ptr;
    forget();
    return ptr;
  }

  @override
  DroppableStaticData get staticData => _staticData;
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class RustArcTransfer<T> {
  final RustArc<T> _arc;
  final RustArc<T> _source;
  final bool _rollback;

  RustArcTransfer._({
    required RustArc<T> arc,
    required RustArc<T> source,
    required bool rollback,
  }) : _arc = arc,
       _source = source,
       _rollback = rollback;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int get ptr => _arc._ptr;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  bool conflictsWith(RustArcTransfer other) =>
      !_rollback && !other._rollback && identical(_source, other._source);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void commit() => _arc.intoRaw();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void rollback() {
    if (_rollback) _arc.dispose();
  }
}

/// Should have exactly *one* instance per *type*.
///
/// For example, all `std::sync::Arc<Apple>` objects should use one
/// `RustArcTypeInfo` object, while all `std::sync::Arc<Orange>`
/// objects should use another.
///
/// The [T] is just a marker type to remind the content type and has no use.
class RustArcStaticData<T> extends DroppableStaticData {
  final RustArcIncrementStrongCountFnType _rustArcIncrementStrongCount;

  /// Constructs the data
  RustArcStaticData({
    /// Directly calls `std::sync::Arc::increment_strong_count(ptr)`
    required RustArcIncrementStrongCountFnType rustArcIncrementStrongCount,

    /// Directly calls `std::sync::Arc::decrement_strong_count(ptr)`
    required RustArcDecrementStrongCountFnType rustArcDecrementStrongCount,

    /// The function pointer to `rustArcDecrementStrongCount`
    required CrossPlatformFinalizerArg rustArcDecrementStrongCountPtr,
  }) : _rustArcIncrementStrongCount = rustArcIncrementStrongCount,
       super(
         releaseFn: rustArcDecrementStrongCount,
         releaseFnPtr: rustArcDecrementStrongCountPtr,
       );
}

/// The type of [RustArcStaticData._rustArcIncrementStrongCount]
typedef RustArcIncrementStrongCountFnType = void Function(PlatformPointer);

/// The type of [RustArcStaticData._rustArcDecrementStrongCount]
typedef RustArcDecrementStrongCountFnType = void Function(PlatformPointer);
