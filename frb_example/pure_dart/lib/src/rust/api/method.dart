// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<SumWith> getSumStruct({dynamic hint}) =>
    RustLib.instance.api.getSumStruct(hint: hint);

Future<SumWithArray3> getSumArray(
        {required int a, required int b, required int c, dynamic hint}) =>
    RustLib.instance.api.getSumArray(a: a, b: b, c: c, hint: hint);

class ConcatenateWith {
  final String a;

  const ConcatenateWith({
    required this.a,
  });

  Future<String> concatenate({required String b, dynamic hint}) =>
      RustLib.instance.api.concatenateWithConcatenate(
        that: this,
        b: b,
      );

  static Future<String> concatenateStatic(
          {required String a, required String b, dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithConcatenateStatic(a: a, b: b, hint: hint);

  static Stream<Log2> handleSomeStaticStreamSink(
          {required int key, required int max, dynamic hint}) =>
      RustLib.instance.api.concatenateWithHandleSomeStaticStreamSink(
          key: key, max: max, hint: hint);

  static Stream<int> handleSomeStaticStreamSinkSingleArg({dynamic hint}) =>
      RustLib.instance.api
          .concatenateWithHandleSomeStaticStreamSinkSingleArg(hint: hint);

  Stream<Log2> handleSomeStreamSink(
          {required int key, required int max, dynamic hint}) =>
      RustLib.instance.api.concatenateWithHandleSomeStreamSink(
        that: this,
        key: key,
        max: max,
      );

  Stream<int> handleSomeStreamSinkAt1({dynamic hint}) =>
      RustLib.instance.api.concatenateWithHandleSomeStreamSinkAt1(
        that: this,
      );

  static Future<ConcatenateWith> newConcatenateWith(
          {required String a, dynamic hint}) =>
      RustLib.instance.api.concatenateWithNew(a: a, hint: hint);

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConcatenateWith &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class Log2 {
  final int key;
  final String value;

  const Log2({
    required this.key,
    required this.value,
  });

  @override
  int get hashCode => key.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Log2 &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          value == other.value;
}

class SumWith {
  final int x;

  const SumWith({
    required this.x,
  });

  Future<int> sum({required int y, required int z, dynamic hint}) =>
      RustLib.instance.api.sumWithSum(
        that: this,
        y: y,
        z: z,
      );

  @override
  int get hashCode => x.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SumWith && runtimeType == other.runtimeType && x == other.x;
}

class SumWithArray3 extends NonGrowableListView<SumWith> {
  static const arraySize = 3;
  SumWithArray3(List<SumWith> inner)
      : assert(inner.length == arraySize),
        super(inner);
  SumWithArray3.unchecked(List<SumWith> inner) : super(inner);
  SumWithArray3.init(SumWith fill)
      : super(List<SumWith>.filled(arraySize, fill));
}
