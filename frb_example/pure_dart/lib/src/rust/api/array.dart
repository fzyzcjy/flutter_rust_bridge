// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<U8Array5> getArray({dynamic hint}) =>
    RustLib.instance.api.getArray(hint: hint);

Future<PointArray2> getComplexArray({dynamic hint}) =>
    RustLib.instance.api.getComplexArray(hint: hint);

Future<MessageId> newMsgid({required U8Array32 id, dynamic hint}) =>
    RustLib.instance.api.newMsgid(id: id, hint: hint);

Future<U8Array32> useMsgid({required MessageId id, dynamic hint}) =>
    RustLib.instance.api.useMsgid(id: id, hint: hint);

Future<Blob> boxedBlob({required U8Array1600 blob, dynamic hint}) =>
    RustLib.instance.api.boxedBlob(blob: blob, hint: hint);

Future<U8Array1600> useBoxedBlob({required Blob blob, dynamic hint}) =>
    RustLib.instance.api.useBoxedBlob(blob: blob, hint: hint);

Future<FeedId> returnBoxedFeedId({required U8Array8 id, dynamic hint}) =>
    RustLib.instance.api.returnBoxedFeedId(id: id, hint: hint);

Future<U8Array8> returnBoxedRawFeedId({required FeedId id, dynamic hint}) =>
    RustLib.instance.api.returnBoxedRawFeedId(id: id, hint: hint);

Future<TestId> testId({required TestId id, dynamic hint}) =>
    RustLib.instance.api.testId(id: id, hint: hint);

Future<double> lastNumber({required F64Array16 array, dynamic hint}) =>
    RustLib.instance.api.lastNumber(array: array, hint: hint);

Future<TestIdArray2> nestedId({required TestIdArray4 id, dynamic hint}) =>
    RustLib.instance.api.nestedId(id: id, hint: hint);

class Blob {
  final U8Array1600 field0;

  const Blob({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Blob &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class F64Array16 extends NonGrowableListView<double> {
  static const arraySize = 16;
  F64Array16(Float64List inner)
      : assert(inner.length == arraySize),
        super(inner);
  F64Array16.unchecked(Float64List inner) : super(inner);
  F64Array16.init() : super(Float64List(arraySize));
}

class FeedId {
  final U8Array8 field0;

  const FeedId({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeedId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class I32Array2 extends NonGrowableListView<int> {
  static const arraySize = 2;
  I32Array2(Int32List inner)
      : assert(inner.length == arraySize),
        super(inner);
  I32Array2.unchecked(Int32List inner) : super(inner);
  I32Array2.init() : super(Int32List(arraySize));
}

class MessageId {
  final U8Array32 field0;

  const MessageId({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MessageId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class Point {
  final double x;
  final double y;

  const Point({
    required this.x,
    required this.y,
  });

  @override
  int get hashCode => x.hashCode ^ y.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Point &&
          runtimeType == other.runtimeType &&
          x == other.x &&
          y == other.y;
}

class PointArray2 extends NonGrowableListView<Point> {
  static const arraySize = 2;
  PointArray2(List<Point> inner)
      : assert(inner.length == arraySize),
        super(inner);
  PointArray2.unchecked(List<Point> inner) : super(inner);
  PointArray2.init(Point fill) : super(List<Point>.filled(arraySize, fill));
}

class TestId {
  final I32Array2 field0;

  const TestId({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TestId &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class TestIdArray2 extends NonGrowableListView<TestId> {
  static const arraySize = 2;
  TestIdArray2(List<TestId> inner)
      : assert(inner.length == arraySize),
        super(inner);
  TestIdArray2.unchecked(List<TestId> inner) : super(inner);
  TestIdArray2.init(TestId fill) : super(List<TestId>.filled(arraySize, fill));
}

class TestIdArray4 extends NonGrowableListView<TestId> {
  static const arraySize = 4;
  TestIdArray4(List<TestId> inner)
      : assert(inner.length == arraySize),
        super(inner);
  TestIdArray4.unchecked(List<TestId> inner) : super(inner);
  TestIdArray4.init(TestId fill) : super(List<TestId>.filled(arraySize, fill));
}

class U8Array1600 extends NonGrowableListView<int> {
  static const arraySize = 1600;
  U8Array1600(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array1600.unchecked(Uint8List inner) : super(inner);
  U8Array1600.init() : super(Uint8List(arraySize));
}

class U8Array32 extends NonGrowableListView<int> {
  static const arraySize = 32;
  U8Array32(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array32.unchecked(Uint8List inner) : super(inner);
  U8Array32.init() : super(Uint8List(arraySize));
}

class U8Array5 extends NonGrowableListView<int> {
  static const arraySize = 5;
  U8Array5(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array5.unchecked(Uint8List inner) : super(inner);
  U8Array5.init() : super(Uint8List(arraySize));
}

class U8Array8 extends NonGrowableListView<int> {
  static const arraySize = 8;
  U8Array8(Uint8List inner)
      : assert(inner.length == arraySize),
        super(inner);
  U8Array8.unchecked(Uint8List inner) : super(inner);
  U8Array8.init() : super(Uint8List(arraySize));
}
