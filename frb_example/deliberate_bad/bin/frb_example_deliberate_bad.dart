import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';
import 'package:frb_example_deliberate_bad/src/rust/api/simple.dart';
import 'package:frb_example_deliberate_bad/src/rust/frb_generated.dart';

Future<void> main(List<String> args) async {
  await RustLib.init();

  switch (args[0]) {
    case 'DartOnly_Good':
      print('I am good Dart code');

    case 'DartOnly_HeapUseAfterFree':
      // ref https://github.com/dart-lang/sdk/blob/main/tests/ffi/calloc_test.dart
      // https://github.com/dart-lang/samples/blob/main/ffi/structs/structs.dart
      final p = calloc<ffi.Float>();
      print('read p firstly: ${p.value}');
      calloc.free(p);
      print('read p after free: ${p.value}');

    case 'DartOnly_MemoryLeak':
      final p = calloc<ffi.Float>();
      print('read p: ${p.value}');

    case 'DartCallRust_StackBufferOverflow':
      await makeStackBufferOverflow();

    case 'DartCallRust_HeapUseAfterFree':
      await makeHeapUseAfterFree();

    case 'DartCallRust_UseOfUninitializedValue':
      await makeUseOfUninitializedValue();

    case 'DartCallRust_MemoryLeak':
      await makeMemoryLeak();

    case 'DartCallRust_DataRace':
      await makeDataRace();

    default:
      throw Exception('Unknown args $args');
  }
}
