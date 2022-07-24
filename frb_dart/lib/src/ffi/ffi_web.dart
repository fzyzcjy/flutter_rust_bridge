import 'package:js/js.dart';
export 'package:js/js.dart';
export 'package:js/js_util.dart' show promiseToFuture;
import 'dart:typed_data';

@JS('Function')
class _UnaryFunction {
  external dynamic call();
  external factory _UnaryFunction(String script);
}

dynamic eval(String script) => _UnaryFunction(script)();

abstract class FlutterRustBridgeWireBase {
  void storeDartPostCObject() {}
  void free_WireSyncReturnStruct(WireSyncReturnStruct raw) {}
}

@JS()
@anonymous
class WireSyncReturnStruct {
  external final Uint8List buffer;
  external final int success;
}

extension WireSyncReturnStructExt on WireSyncReturnStruct {
  bool get isSuccess => success > 0;
}
