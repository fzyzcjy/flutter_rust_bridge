// TODO this is manually written to prototype the API

/// Main entrypoint of the Rust API
class FrbExamplePureDart {
  static FrbExamplePureDartDispatcher get dispatcher =>
      _dispatcher ?? (throw StateError('flutter_rust_bridge has not been initialized'));
  static FrbExamplePureDartDispatcher? _dispatcher;

  static Future<void> init({
    FrbExamplePureDartDispatcher? dispatcher,
  }) async {
    _dispatcher = dispatcher ?? FrbExamplePureDartDispatcher();
    // TODO real initialization work
  }
}

class FrbExamplePureDartDispatcher {
  final FlutterRustBridgeHandler? handler;

  FrbExamplePureDartDispatcher({this.handler});

// TODO auto gen code
}

// TODO move to runtime lib
abstract class FlutterRustBridgeHandler {}

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
