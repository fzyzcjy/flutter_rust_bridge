import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';

/// The Rust `std::Vec<u8>` on the Dart side.
/// Must call `dispose` manually, otherwise the memory will be leaked.
class RustVecU8 {
  final GeneralizedFrbRustBinding _binding;

  /// Null = already disposed (to avoid accidential double free)
  ffi.Pointer<ffi.Uint8>? _ptr;

  /// {@macro flutter_rust_bridge.internal}
  final int length;

  /// {@macro flutter_rust_bridge.internal}
  RustVecU8(this.length, this._binding) : _ptr = _binding.rustVecU8New(length);

  /// {@macro flutter_rust_bridge.internal}
  void dispose() {
    final ptr = _ptr!;
    _ptr = null;
    // Set ptr to null before calling free to avoid potential
    // double-free when error happens
    _binding.rustVecU8Free(ptr, length);
  }
}
