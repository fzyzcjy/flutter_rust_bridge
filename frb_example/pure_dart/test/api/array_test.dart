import 'package:frb_example_pure_dart/src/rust/api/array.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call getArray()', () async {
    var array = await getArray();
    expect(array, [1, 2, 3, 4, 5]);
  });

  test('dart call getComplexArray()', () async {
    final points = await getComplexArray();

    expect(points[0].x, 1.0);
    expect(points[1].x, 2.0);
  });

  test('MessageId', () async {
    final MessageId msgid = await newMsgid(id: U8Array32.init());
    msgid.field0[2] = 14;
    final inner = await useMsgid(id: msgid);
    expect(inner[2], 14);
  });
  test('BlobId', () async {
    final inner = U8Array1600.init();
    inner[14] = 99;
    final Blob blob = await boxedBlob(blob: inner);
    expect(blob.field0[14], 99);
    blob.field0[10] = 100;
    final unboxed = await useBoxedBlob(blob: blob);
    expect(unboxed[10], 100);
    expect(unboxed[14], 99);
  });
  test('FeedId', () async {
    final inner = U8Array8.init();
    inner[3] = 3;
    final FeedId feedId = await returnBoxedFeedId(id: inner);
    expect(feedId.field0[3], 3);
    feedId.field0[5] = 5;
    final raw = await returnBoxedRawFeedId(id: feedId);
    expect(raw[5], 5);
    expect(raw[3], 3);
  });
  test('TestId', () async {
    final inner = I32Array2.init();
    inner[0] = 1;
    inner[1] = 2;
    final testId = await testId(id: TestId(field0: inner));
    expect(testId.field0[0], 1);
    expect(testId.field0[1], 2);
  });
  test('LastNumber', () async {
    final array = F64Array16.init();
    array[15] = 88.888;
    final result = await lastNumber(array: array);
    expect(result, 88.888);
  });
  test('NestedId', () async {
    final id0 = TestId(field0: I32Array2.init());
    id0.field0[1] = 10;
    final id1 = TestId(field0: I32Array2.init());
    id1.field0[1] = 20;
    final id2 = TestId(field0: I32Array2.init());
    id2.field0[1] = 30;
    final id3 = TestId(field0: I32Array2.init());
    id3.field0[1] = 40;
    final x = await nestedId(id: TestIdArray4([id0, id1, id2, id3]));
    expect(x[0].field0[1], 10);
    expect(x[1].field0[1], 40);
  });
}
