import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/primitive_list_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  // test('dart call simpleAdder', () async {
  //   expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
  // });

  print('TODO temp');

  test('hi', () async {
    expect(
        examplePrimitiveListTypeU8TwinSyncSse(arg: Uint8List.fromList([255])),
        255);
   
    throw Exception('TODO');
  });
}
