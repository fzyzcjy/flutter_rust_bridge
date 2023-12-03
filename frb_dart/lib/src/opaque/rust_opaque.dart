import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/rust_arc/_common.dart';
import 'package:meta/meta.dart';

/// An opaque pointer to a native arbitrary Rust type.
/// TODO: link to the doc talking about "dispose"/GC semantics
abstract class RustOpaque {
  final RustArc _arc;

  /// If true, when sending this object to Rust, the ownership of this object
  /// will be released, mimicking the "move" semantics in Rust.
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
    return target.intoRaw();
  }

  /// Dispose the underlying `Arc`.
  void dispose() => _arc.dispose();

  /// Whether the underlying `Arc` is disposed.
  bool get isDisposed => _arc.isDisposed;
}
