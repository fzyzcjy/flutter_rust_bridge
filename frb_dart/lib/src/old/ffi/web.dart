import 'package:js/js.dart';

export 'package:js/js.dart';
export 'package:js/js_util.dart' show promiseToFuture, getProperty;

@JS('Number')
external int castInt(Object? value);
