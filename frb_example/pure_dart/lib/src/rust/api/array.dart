// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
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
