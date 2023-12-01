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

  void wire_make_data_race(NativePortType port_) =>
      wasmModule.wire_make_data_race(port_);

  void wire_make_heap_use_after_free(NativePortType port_) =>
      wasmModule.wire_make_heap_use_after_free(port_);

  void wire_make_memory_leak(NativePortType port_) =>
      wasmModule.wire_make_memory_leak(port_);

  void wire_make_stack_buffer_overflow(NativePortType port_) =>
      wasmModule.wire_make_stack_buffer_overflow(port_);

  void wire_make_use_of_uninitialized_value(NativePortType port_) =>
      wasmModule.wire_make_use_of_uninitialized_value(port_);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);

  external RustLibWasmModule bind(dynamic thisArg, String moduleName);

  external void wire_make_data_race(NativePortType port_);

  external void wire_make_heap_use_after_free(NativePortType port_);

  external void wire_make_memory_leak(NativePortType port_);

  external void wire_make_stack_buffer_overflow(NativePortType port_);

  external void wire_make_use_of_uninitialized_value(NativePortType port_);
}
