import 'package:js/js.dart';

@JS('console.warn')
external void jsConsoleWarn([a, b, c, d, e, f, g, h, i]);

@JS('Function')
class _Function {
  external dynamic call();

  external factory _Function(String script);
}

dynamic jsEval(String script) => _Function(script)();
