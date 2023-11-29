// ignore_for_file: unused_import, unused_element, duplicate_ignore

import 'api/simple.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.dropPortManager,
  });
}

// Section: wire_class

class RustLibWire extends BaseWire {
  // TODO
  // : super(WasmModule.cast<RustLibWasmModule>(lib.wasmModule));
  RustLibWire.fromExternalLibrary(ExternalLibrary lib) {}

  void wire_add(NativePortType port_, int left, int right) =>
      wasmModule.wire_add(port_, left, right);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);

  external RustLibWasmModule bind(dynamic thisArg, String moduleName);

  external void wire_add(NativePortType port_, int left, int right);
}
