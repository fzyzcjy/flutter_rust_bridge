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

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  Uint8List api2wire_list_prim_u_8(Uint8List raw) {
    return raw;
  }
}

// Section: wire_class

class RustLibWire extends BaseWire {
  RustLibWire.fromExternalLibrary(ExternalLibrary lib);

  dynamic /* flutter_rust_bridge::support::WireSyncReturn */ wire_greet(
          String name) =>
      wasmModule.wire_greet(name);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule implements WasmModule {
  @override
  external Object /* Promise */ call([String? moduleName]);

  @override
  external RustLibWasmModule bind(dynamic thisArg, String moduleName);

  external dynamic /* flutter_rust_bridge::support::WireSyncReturn */
      wire_greet(String name);
}
