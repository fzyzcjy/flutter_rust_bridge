import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'frb_generated.io.dart'
    if (dart.library.html) 'frb_generated.web.dart.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
  }) async {
    await instance.initImpl(api: api, handler: handler);
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
  WireConstructor<RustLibWire> get wireConstructor => RustLibWire.new;

  @override
  String get defaultExternalLibraryStem => 'frb_example_pure_dart';

  @override
  String get defaultExternalLibraryRelativeDirectory => 'rust/target/release/';
}

abstract class RustLibApi extends BaseApi {
  Future<int> anotherAdder({required int a, required int b, dynamic hint});
  Future<int> simpleAdder({required int a, required int b, dynamic hint});
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
  });

  @override
  Future<int> anotherAdder({required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_another_adder(port_, arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kAnotherAdderConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kAnotherAdderConstMeta => const TaskConstMeta(
        debugName: "another_adder",
        argNames: ["a", "b"],
      );
  @override
  Future<int> simpleAdder({required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_simple_adder(port_, arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kSimpleAdderConstMeta,
      argValues: [a, b],
      apiImpl: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSimpleAdderConstMeta => const TaskConstMeta(
        debugName: "simple_adder",
        argNames: ["a", "b"],
      );
}

// Section: boilerplate
// Section: api_impl_normal_functions
// Section: api_impl_opaque_getters
// Section: c_binding
// Section: impl_wire2api
int _wire2api_i_32(dynamic raw) {
  return raw as int;
}

// Section: api2wire_funcs
@protected
int api2wire_i_32(int raw) {
  return raw;
} // Section: api_fill_to_wire_funcs
