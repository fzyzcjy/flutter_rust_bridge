// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<StructWithOneField> funcStructWithOneFieldTwinNormal(
        {required StructWithOneField arg, dynamic hint}) =>
    RustLib.instance.api.funcStructWithOneFieldTwinNormal(arg: arg, hint: hint);

Future<StructWithTwoField> funcStructWithTwoFieldTwinNormal(
        {required StructWithTwoField arg, dynamic hint}) =>
    RustLib.instance.api.funcStructWithTwoFieldTwinNormal(arg: arg, hint: hint);

Future<StructWithZeroField> funcStructWithZeroFieldTwinNormal(
        {required StructWithZeroField arg, dynamic hint}) =>
    RustLib.instance.api
        .funcStructWithZeroFieldTwinNormal(arg: arg, hint: hint);

Future<TupleStructWithOneField> funcTupleStructWithOneFieldTwinNormal(
        {required TupleStructWithOneField arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithOneFieldTwinNormal(arg: arg, hint: hint);

Future<TupleStructWithTwoField> funcTupleStructWithTwoFieldTwinNormal(
        {required TupleStructWithTwoField arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithTwoFieldTwinNormal(arg: arg, hint: hint);

class StructWithOneField {
  final int a;

  const StructWithOneField({
    required this.a,
  });
}

class StructWithTwoField {
  final int a;
  final int b;

  const StructWithTwoField({
    required this.a,
    required this.b,
  });
}

class StructWithZeroField {
  const StructWithZeroField();
}

class TupleStructWithOneField {
  final int field0;

  const TupleStructWithOneField({
    required this.field0,
  });
}

class TupleStructWithTwoField {
  final int field0;
  final int field1;

  const TupleStructWithTwoField({
    required this.field0,
    required this.field1,
  });
}
