// ignore_for_file: non_constant_identifier_names

@JS()
library flutter_rust_bridge_web;

import 'package:flutter_rust_bridge/src/universal_isolate.dart' show pm;
import 'package:js/js.dart';

export 'src/basic.dart';
export 'src/helpers.dart';

@JS("window")
external dynamic get _window;

@JS("Object.assign")
external void _assign(dynamic target, dynamic source);

typedef _Send = bool Function(int port, Object? arg);

@JS()
@anonymous
class FrbWindowExt {
  external _Send get frb_post_js_object;
  external factory FrbWindowExt({_Send frb_post_js_object});
}

void setup() {
  _assign(_window, FrbWindowExt(frb_post_js_object: allowInterop(pm.send)));
}
