import 'dart:async';
import 'dart:ffi' as ffi;
export 'dart:ffi' show NativePort, DynamicLibrary;

import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

/// Abstraction over a Dart SendPort and a JS MessagePort.
typedef NativePortType = int;
typedef ExternalLibrary = ffi.DynamicLibrary;
Future<dynamic> promiseToFuture(Object promise) =>
    throw PlatformMismatchException();
dynamic eval(String jsScript) => throw PlatformMismatchException();

const nativeLog = print;

/// Whether the web platform has been isolated by COOP and COEP headers,
/// and is capable of sharing buffers between workers.
///
/// Note: not available on all browsers, in which case it will return null.
bool? get crossOriginIsolated => throw PlatformMismatchException();

class JS {
  /// Dummy JS attribute.
  const JS([String? name]);
}

/// Dummy anonymous attribute.
const anonymous = Object();

/// This class, together with its subclasses, are only for internal usage.
/// Usually it should not be used by normal users.
abstract class FlutterRustBridgeWireBase {
  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(
    ffi.Pointer<
            ffi.NativeFunction<
                ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>
        ptr,
  ) {}

  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void free_WireSyncReturnStruct(WireSyncReturnStruct val) {}
}

abstract class FlutterRustBridgeWasmWireBase<T extends WasmModule>
    extends FlutterRustBridgeWireBase {
  Future<T> get init => Future.error(PlatformMismatchException());
  FlutterRustBridgeWasmWireBase([FutureOr<T>? _]);
}

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() {
    store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
  }
}

// NOTE for maintainer: Please manually keep in sync with [WireSyncReturnStruct] in Rust
/// This class is only for internal usage.
class WireSyncReturnStruct extends ffi.Struct {
  /// Not to be used by normal users, but has to be public for generated code
  external ffi.Pointer<ffi.Uint8> ptr;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Int32()
  external int len;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Uint8()
  external int success;

  Uint8List get buffer => Uint8List.fromList(ptr.asTypedList(len));
  bool get isSuccess => success > 0;
}
