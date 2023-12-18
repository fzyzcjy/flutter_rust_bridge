import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/primitive_list_misc.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handleVecOfPrimitive', () async {
    final n = 10000;
    final resp = await handleVecOfPrimitiveTwinNormal(n: n);
    expect(resp.uint8List, Uint8List.fromList(List.filled(n, 42)));
    expect(resp.int8List, Int8List.fromList(List.filled(n, 42)));
    expect(resp.uint16List, Uint16List.fromList(List.filled(n, 42)));
    expect(resp.int16List, Int16List.fromList(List.filled(n, 42)));
    expect(resp.uint32List, Uint32List.fromList(List.filled(n, 42)));
    expect(resp.int32List, Int32List.fromList(List.filled(n, 42)));
    expect(resp.float32List, Float32List.fromList(List.filled(n, 42)));
    expect(resp.float64List, Float64List.fromList(List.filled(n, 42)));
    expect(resp.uint64List, Uint64List.fromList(List.filled(n, 42)));
    expect(resp.int64List, Int64List.fromList(List.filled(n, 42)));
    expect(resp.boolList, List.filled(n, true));
  });

  // no longer needed, since auto enable zero copy
  // test('dart call handleZeroCopyVecOfPrimitive', () async {
  //   final n = 10000;
  //   final resp = await handleZeroCopyVecOfPrimitiveTwinNormal(n: n);
  //   expect(resp.uint8List, Uint8List.fromList(List.filled(n, 42)));
  //   expect(resp.int8List, Int8List.fromList(List.filled(n, 42)));
  //   expect(resp.uint16List, Uint16List.fromList(List.filled(n, 42)));
  //   expect(resp.int16List, Int16List.fromList(List.filled(n, 42)));
  //   expect(resp.uint32List, Uint32List.fromList(List.filled(n, 42)));
  //   expect(resp.int32List, Int32List.fromList(List.filled(n, 42)));
  //   expect(resp.float32List, Float32List.fromList(List.filled(n, 42)));
  //   expect(resp.float64List, Float64List.fromList(List.filled(n, 42)));
  //   expect(resp.uint64List, Uint64List.fromList(List.filled(n, 42)));
  //   expect(resp.int64List, Int64List.fromList(List.filled(n, 42)));
  // });
}
