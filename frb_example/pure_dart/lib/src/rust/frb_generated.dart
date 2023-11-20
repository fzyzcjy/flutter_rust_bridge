import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart.dart';

/// Main entrypoint of the Rust API
class Rust extends BaseEntrypoint<RustApi> {
  @internal
  static final instance = Rust._();

  Rust._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustApi? api,
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
  RustApi createDefaultApi({BaseHandler? handler}) =>
      RustApi(apiImpl: RustApiImpl(wire: RustWire(dynamicLibrary), handler: handler));
}

class RustApi extends BaseApi<RustApiImpl> {
  final RustApiImpl apiImpl;

  RustApi({required this.apiImpl});

  Future<int> simpleAdder({required int a, required int b, dynamic hint}) {
    return apiImpl.simpleAdder(a: a, b: b, hint: hint);
  }
}

class RustApiImpl extends RustApiImplPlatform {
  RustApiImpl({super.handler, required super.wire});

  Future<int> simpleAdder({required int a, required int b, dynamic hint}) {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => wire.wire_simple_adder(port_, arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kSimpleAdderConstMeta,
      argValues: [a, b],
      api: this,
      hint: hint,
    ));
  }

  TaskConstMeta get kSimpleAdderConstMeta => const TaskConstMeta(
        debugName: "simple_adder",
        argNames: ["a", "b"],
      );

  int _wire2api_i_32(dynamic raw) {
    return raw as int;
  }

  @protected
  int api2wire_i_32(int raw) {
    return raw;
  }
}
