// TODO this is manually written to prototype the API

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

/// Main entrypoint of the Rust API
class Rust extends BaseEntrypoint<RustDispatcher> {
  @internal
  static final instance = Rust._();

  Rust._();

  static Future<void> init({
    RustDispatcher? dispatcher,
  }) async {
    await instance.initImpl(dispatcher: dispatcher ?? RustDispatcher());
  }
}

class RustDispatcher extends BaseDispatcher {
  RustDispatcher({super.handler});

  Future<int> simpleAdder({required int a, required int b, dynamic hint}) async {
    var arg0 = api2wire_i_32(a);
    var arg1 = api2wire_i_32(b);
    return handler.executeNormal(NormalTask(
      callFfi: (port_) => _platform.inner.wire_simple_adder(port_, arg0, arg1),
      parseSuccessData: _wire2api_i_32,
      parseErrorData: null,
      constMeta: kSimpleAdderConstMeta,
      argValues: [a, b],
      hint: hint,
    ));
  }
}

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
