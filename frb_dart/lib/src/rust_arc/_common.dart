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
      RustArcBase.finalizerAttach(this, _ptr, size, staticFinalizer);
    }
  }

  // TODO refactor?
  // TODO comments
  /// A native finalizer rust opaque type.
  /// It should be *static* for each subtype.
  @protected
  ArcTypeFinalizer get staticFinalizer;

  // TODO rename: dropFn -> rust_arc_decrement_strong_count
  // TODO comments
  /// Rust type specific drop function.
  ///
  /// This function should never be called manually.
  @protected
  void staticRustArcDecrementStrongCount(PlatformPointer ptr);

  // TODO rename: shareFn -> rust_arc_increment_strong_count
  // TODO comments
  /// Rust type specific share function.
  ///
  /// This function should never be called manually.
  @protected
  void staticRustArcIncrementStrongCount(PlatformPointer ptr);
}
