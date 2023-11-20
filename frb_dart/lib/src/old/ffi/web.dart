import 'package:js/js.dart';

export 'package:js/js.dart';
export 'package:js/js_util.dart' show promiseToFuture, getProperty;

@JS()
external bool? get crossOriginIsolated;

@JS('Number')
external int castInt(Object? value);

@JS('BigInt')
external Object castNativeBigInt(Object? value);

abstract class DartApiDl {}
