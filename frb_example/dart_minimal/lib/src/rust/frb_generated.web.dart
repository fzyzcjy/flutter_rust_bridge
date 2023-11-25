// ignore_for_file: unused_import, unused_element

import 'api/minimal.dart';
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

  void wire_minimal_adder(NativePortType port_, int a, int b) =>
      wasmModule.wire_minimal_adder(port_, a, b);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);

  external RustLibWasmModule bind(dynamic thisArg, String moduleName);

  external void wire_minimal_adder(NativePortType port_, int a, int b);
}
