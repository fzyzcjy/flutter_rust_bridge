import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_io.dart'
    if (dart.library.html) '_web.dart';
import 'package:meta/meta.dart';

// TODO
/// The type of [RustArc] drop function
typedef ArcDropFnType = void Function(PlatformPointer);

// TODO
/// The type of [RustArc] share function
typedef ArcShareFnType = PlatformPointer Function(PlatformPointer);

/// The Rust `std::sync::Arc` on the Dart side.
abstract class RustArc extends RustArcBase {
  /// Either the pointer that `std::sync::Arc::into_raw` gives,
  /// or a null pointer.
  ///
  /// In other words, when non-null, it is very similar to `std::sync::Arc.ptr`,
  /// but only with a small constant offset.
  PlatformPointer _ptr;

  /// Mimic `std::sync::Arc::from_raw`
  RustArc.fromRaw({required int ptr, required int size})
      : _ptr = PlatformPointerUtil.ptrFromInt(ptr) {
    if (!PlatformPointerUtil.isNullPtr(_ptr)) {
      RustArcBase.finalizerAttach(this, _ptr, size, typeInfo.finalizer);
    }
  }

  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has
  /// actually been reclaimed.
  bool isDisposed() => PlatformPointerUtil.isNullPtr(_ptr);

  /// See comments in [RustArcPerTypeData] for details.
  @protected
  RustArcPerTypeData get typeInfo;
}

/// Should have exactly *one* instance per *type*.
///
/// For example, all `std::sync::Arc<Apple>` objects should use one
/// `RustArcTypeInfo` object, while all `std::sync::Arc<Orange>`
/// objects should use another.
abstract class RustArcPerTypeData {
  // TODO refactor?
  // TODO comments
  /// Finalizer for the subtype.
  // TODO can we ensure this static?
  /// It should be static for each subtype.
  @protected
  ArcTypeFinalizer get finalizer;

  // TODO rename: dropFn -> rust_arc_decrement_strong_count
  // TODO comments
  /// Directly calls `std::sync::Arc::decrement_strong_count(ptr)`
  @protected
  void rustArcDecrementStrongCount(PlatformPointer ptr);

  // TODO rename: shareFn -> rust_arc_increment_strong_count
  // TODO comments
  /// Directly calls `std::sync::Arc::increment_strong_count(ptr)`
  @protected
  void rustArcIncrementStrongCount(PlatformPointer ptr);
}
