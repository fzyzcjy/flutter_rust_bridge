// ignore_for_file: unused_import, unused_element, duplicate_ignore

import 'dart:async';
import 'dart:convert';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'api/simple.dart';
import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
  }) async {
    await instance.initImpl(
      api: api,
      handler: handler,
      externalLibrary: externalLibrary,
    );
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      const ExternalLibraryLoaderConfig(
        stem: 'frb_example_deliberate_bad',
        ioDirectory: 'rust/target/release/',
        webPrefix: 'pkg/',
      );
}

abstract class RustLibApi extends BaseApi {
  Future<void> makeHeapUseAfterFree({dynamic hint});

  Future<void> makeStackBufferOverflow({dynamic hint});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.dropPortManager,
  });

  @override
  Future<void> makeHeapUseAfterFree({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_make_heap_use_after_free(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kMakeHeapUseAfterFreeConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMakeHeapUseAfterFreeConstMeta => const TaskConstMeta(
        debugName: "make_heap_use_after_free",
        argNames: [],
      );

  @override
  Future<void> makeStackBufferOverflow({dynamic hint}) {
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_make_stack_buffer_overflow(port_),
      parseSuccessData: _wire2api_unit,
      parseErrorData: null,
      constMeta: kMakeStackBufferOverflowConstMeta,
      argValues: [],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kMakeStackBufferOverflowConstMeta => const TaskConstMeta(
        debugName: "make_stack_buffer_overflow",
        argNames: [],
      );

  void _wire2api_unit(dynamic raw) {
    return;
  }
}
