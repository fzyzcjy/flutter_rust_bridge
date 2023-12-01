import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';

void main(List<String> args) {
  switch (args[0]) {
    case 'DartOnly_Good':
      print('I am good Dart code');

    case 'DartOnly_HeapUseAfterFree':
      // ref https://github.com/dart-lang/sdk/blob/main/tests/ffi/calloc_test.dart
      // https://github.com/dart-lang/samples/blob/main/ffi/structs/structs.dart
      final p = calloc<ffi.Float>();
      print('read p (good): ${p.value}');
      calloc.free(p);
      print('read p after free (should error): ${p.value}');

    default:
      throw Exception('Unknown args $args');
  }
}
