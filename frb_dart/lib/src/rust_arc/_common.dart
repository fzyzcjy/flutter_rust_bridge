import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_io.dart'
    if (dart.library.html) '_web.dart';

// TODO
/// The type of [RustArc] drop function
typedef ArcDropFnType = void Function(PlatformPointer);

// TODO
/// The type of [RustArc] share function
typedef ArcShareFnType = PlatformPointer Function(PlatformPointer);

/// The Rust `std::sync::Arc` on the Dart side.
abstract class RustArc extends RustArcBase {
  /// The pointer that `std::sync::Arc::into_raw` gives.
  ///
  /// In other words, it is very similar to `std::sync::Arc.ptr`, but only
  /// with a small constant offset.
  PlatformPointer _ptr;

  /// Mimic `std::sync::Arc::from_raw`
  RustArc.fromRaw({required int ptr, required int size})
      : _ptr = PlatformPointerUtil.ptrFromInt(ptr) {
    if (ptr != 0) {
      RustArcBase.finalizerAttach(this, _ptr, size, staticFinalizer);
    }
  }

  // TODO refactor?
  // TODO comments
  /// A native finalizer rust opaque type.
  /// It is static for each frb api class instance.
  ArcTypeFinalizer get staticFinalizer;

  // TODO rename
  // TODO comments
  /// Rust type specific drop function.
  ///
  /// This function should never be called manually.
  ArcDropFnType get dropFn;

  // TODO rename
  // TODO comments
  /// Rust type specific share function.
  ///
  /// This function should never be called manually.
  ArcShareFnType get shareFn;
}
