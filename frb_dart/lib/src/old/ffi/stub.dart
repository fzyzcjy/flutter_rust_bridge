import 'dart:async';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

import 'io.dart' if (dart.library.html) 'web.dart' show DartPostCObject;

export 'io.dart' if (dart.library.html) 'web.dart' show ExternalLibrary, DartApiDl;

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() => throw UnimplementedError();
}

/// Whether the web platform has been isolated by COOP and COEP headers,
/// and is capable of sharing buffers between workers.
///
/// Note: not available on all browsers, in which case it will return null.
bool? get crossOriginIsolated => throw UnimplementedError();

int castInt(Object? value) => value as int;

/// Only used on the Web.
Object castNativeBigInt(int value) => throw UnimplementedError();

abstract class FlutterRustBridgeWasmWireBase<T extends WasmModule> extends FlutterRustBridgeWireBase {
  Future<T> get init => throw UnimplementedError();

  FlutterRustBridgeWasmWireBase([FutureOr<T>? module]);
}

class JS {
  const JS([String? name]);
}

class _Anonymous {
  const _Anonymous();
}

const anonymous = _Anonymous();
