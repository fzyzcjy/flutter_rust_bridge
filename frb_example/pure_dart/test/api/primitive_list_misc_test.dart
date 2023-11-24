import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/primitive_list_misc.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call handleVecOfPrimitive', () async {
    final n = 10000;
    final resp = await handleVecOfPrimitive(n: n);
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

  // TODO rm?
  // test('dart call handleVecOfPrimitiveSync', () {
  //   final n = 10000;
  //   final resp = handleVecOfPrimitiveSync(n: n);
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
  //   expect(resp.boolList, List.filled(n, true));
  // });

  test('dart call handleZeroCopyVecOfPrimitive', () async {
    final n = 10000;
    final resp = await handleZeroCopyVecOfPrimitive(n: n);
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
  });

  // TODO rm?
  // test('dart call handleZeroCopyVecOfPrimitiveSync', () {
  //   final n = 10000;
  //   final resp = handleZeroCopyVecOfPrimitiveSync(n: n);
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
