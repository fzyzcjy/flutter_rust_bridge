import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_common.dart';
import 'package:meta/meta.dart';

// TODO comments
/// An opaque pointer to a native Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
abstract class RustOpaque {
  final RustArc _arc;

  /// Displays the need to release ownership when sending to rust.
  bool _move = false;

  set move(bool move) => _move = move;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  RustOpaque.fromWire(List<dynamic> wire, RustArcStaticData staticData)
      : _arc = RustArc.fromRaw(
          ptr: wire[0],
          externalSizeOnNative: wire[1],
          staticData: staticData,
        );

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  PlatformPointer api2wire() {
    final target = _move ? _arc : _arc.clone();
    return target.intoRaw() ?? PlatformPointerUtil.nullPtr();
  }

  /// Dispose the underlying `Arc`.
  void dispose() => _arc.dispose();
}
