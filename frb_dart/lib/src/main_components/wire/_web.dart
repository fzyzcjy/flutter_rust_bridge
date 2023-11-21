import 'dart:async';
import 'dart:js_util';

import 'package:flutter_rust_bridge/src/main_components/wire/wire.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class BaseWasmWire<T extends WasmModule> extends BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final Future<T> init;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  BaseWasmWire(FutureOr<T> module) : init = Future.value(module).then((module) => promiseToFuture(module()));
}
