import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/opaque/_io.dart'
    if (dart.library.html) '_web.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:meta/meta.dart';

/// The type of [RustOpaque] drop function
typedef OpaqueDropFnType = void Function(PlatformPointer);

/// The type of [RustOpaque] share function
typedef OpaqueShareFnType = PlatformPointer Function(PlatformPointer);

/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
abstract class RustOpaque extends RustOpaqueBase {
  /// Pointer to this opaque Rust type.
  PlatformPointer _ptr;

  /// A native finalizer rust opaque type.
  /// Is static for each frb api class instance.
  OpaqueTypeFinalizer get staticFinalizer;

  /// Displays the need to release ownership when sending to rust.
  bool _move = false;

  set move(bool move) => _move = move;

  /// Rust type specific drop function.
  ///
  /// This function should never be called manually.
  OpaqueDropFnType get dropFn;

  /// Rust type specific share function.
  ///
  /// This function should never be called manually.
  OpaqueShareFnType get shareFn;

  /// This constructor should never be called manually.
  @internal
  RustOpaque.unsafe(int ptr, int size) : _ptr = RustOpaqueBase.initPtr(ptr) {
    if (ptr != 0) {
      RustOpaqueBase.finalizerAttach(this, _ptr, size, staticFinalizer);
    }
  }

  /// Call Rust destructors on the backing memory of this pointer.
  ///
  /// This function should be run at least once during the lifetime of the
  /// program, and can be run many times.
  ///
  /// When passed into a Rust function, Rust enacts *shared ownership*,
  /// if this pointer is shared with Rust when [dispose] is called,
  /// ownership is fully transferred to Rust else this pointer is cleared.
  void dispose() {
    if (!isStale()) {
      var ptr = _ptr;
      _ptr = RustOpaqueBase.nullPtr();

      staticFinalizer.detach(this);
      dropFn(ptr);
    }
  }

  /// Increments inner reference counter and returns pointer to the underlying
  /// Rust object.
  ///
  /// Throws a [StateError] if called after [dispose].
  @internal
  PlatformPointer shareOrMove() {
    if (!isStale()) {
      var ptr = shareFn(_ptr);
      if (_move) {
        dispose();
      }
      return ptr;
    } else {
      return RustOpaqueBase.nullPtr();
    }
  }

  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has
  /// actually been reclaimed.
  bool isStale() => RustOpaqueBase.isStalePtr(_ptr);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DropPortManager {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final GeneralizedFrbRustBinding _generalizedFrbRustBinding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  DropPortManager(this._generalizedFrbRustBinding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  NativePortType get dropPort => _dropPort.sendPort.nativePort;
  late final _dropPort = _initDropPort();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  ReceivePort _initDropPort() {
    final port = broadcastPort(DropIdPortGenerator.create());
    port.listen((message) {
      _generalizedFrbRustBinding.dropDartObject(message);
    });
    return port;
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dispose() {
    _dropPort.close();
  }
}
