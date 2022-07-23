import 'package:js/js.dart';
import 'dart:typed_data';

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
