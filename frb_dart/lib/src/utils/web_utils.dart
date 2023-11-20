import 'package:js/js.dart';

@JS('console.warn')
external void jsConsoleWarn([a, b, c, d, e, f, g, h, i]);

@JS('Function')
class _Function {
  external dynamic call();

  external factory _Function(String script);
}

dynamic jsEval(String script) => _Function(script)();

/// Whether the web platform has been isolated by COOP and COEP headers,
/// and is capable of sharing buffers between workers.
///
/// Note: not available on all browsers, in which case it will return null.
@JS()
external bool? get crossOriginIsolated;
