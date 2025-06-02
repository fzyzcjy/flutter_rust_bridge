// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT
// FRB_INTERNAL_GENERATOR: {"addCode": "import 'package:frb_example_pure_dart_pde/src/rust/api/array.dart';"}

import 'package:frb_example_pure_dart_pde/src/rust/api/array.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call getArray()', () async {
    var array = await getArrayTwinNormal();
    expect(array, [1, 2, 3, 4, 5]);
  });

  test('dart call getComplexArray()', () async {
    final points = await getComplexArrayTwinNormal();

    expect(points[0].x, 1.0);
    expect(points[1].x, 2.0);
  });

  test('MessageId', () async {
    final MessageIdTwinNormal msgid =
        await newMsgidTwinNormal(id: U8Array32.init());
    msgid.field0[2] = 14;
    final inner = await useMsgidTwinNormal(id: msgid);
    expect(inner[2], 14);
  });

  test('BlobId', () async {
    final inner = U8Array1600.init();
    inner[14] = 99;
    final BlobTwinNormal blob = await boxedBlobTwinNormal(blob: inner);
    expect(blob.field0[14], 99);
    blob.field0[10] = 100;
    final unboxed = await useBoxedBlobTwinNormal(blob: blob);
    expect(unboxed[10], 100);
    expect(unboxed[14], 99);
  });

  test('FeedId', () async {
    final inner = U8Array8.init();
    inner[3] = 3;
    final FeedIdTwinNormal feedId =
        await returnBoxedFeedIdTwinNormal(id: inner);
    expect(feedId.field0[3], 3);
    feedId.field0[5] = 5;
    final raw = await returnBoxedRawFeedIdTwinNormal(id: feedId);
    expect(raw[5], 5);
    expect(raw[3], 3);
  });

  test('TestId', () async {
    final inner = I32Array2.init();
    inner[0] = 1;
    inner[1] = 2;
    final testId =
        await funcTestIdTwinNormal(id: TestIdTwinNormal(field0: inner));
    expect(testId.field0[0], 1);
    expect(testId.field0[1], 2);
  });

  test('LastNumber', () async {
    final array = F64Array16.init();
    array[15] = 88.888;
    final result = await lastNumberTwinNormal(array: array);
    expect(result, 88.888);
  });

  test('NestedId', () async {
    final id0 = TestIdTwinNormal(field0: I32Array2.init());
    id0.field0[1] = 10;
    final id1 = TestIdTwinNormal(field0: I32Array2.init());
    id1.field0[1] = 20;
    final id2 = TestIdTwinNormal(field0: I32Array2.init());
    id2.field0[1] = 30;
    final id3 = TestIdTwinNormal(field0: I32Array2.init());
    id3.field0[1] = 40;
    final x = await nestedIdTwinNormal(
        id: TestIdTwinNormalArray4([id0, id1, id2, id3]));
    expect(x[0].field0[1], 10);
    expect(x[1].field0[1], 40);
  });
}
