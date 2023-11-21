import 'dart:async';
import 'dart:js_util';

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

export '_io.dart' if (dart.library.html) '_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final ExternalLibrary externalLibrary;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseWire({required this.externalLibrary});
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
class BaseWasmWire<T extends WasmModule> extends BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final Future<T> init;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  BaseWasmWire({required super.externalLibrary})
      : init = Future.value(externalLibrary).then((module) => promiseToFuture(module()));
}
