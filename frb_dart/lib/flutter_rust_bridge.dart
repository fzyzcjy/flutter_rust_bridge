import 'package:flutter_rust_bridge/src/basic.dart';

export 'src/flutter_rust_bridge_io.dart'
    if (dart.library.html) 'flutter_rust_bridge_web.dart';
export 'src/platform_independent.dart';

class PanicError extends FrbException {
  String error;
  PanicError(this.error);
}

PanicError wire2apiPanicError(dynamic raw) {
  return PanicError(raw as String);
}
