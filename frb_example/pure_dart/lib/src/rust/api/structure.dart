// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<StructWithOneFieldTwinNormal> funcStructWithOneFieldTwinNormal(
        {required StructWithOneFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcStructWithOneFieldTwinNormal(arg: arg, hint: hint);

Future<StructWithTwoFieldTwinNormal> funcStructWithTwoFieldTwinNormal(
        {required StructWithTwoFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api.funcStructWithTwoFieldTwinNormal(arg: arg, hint: hint);

Future<StructWithZeroFieldTwinNormal> funcStructWithZeroFieldTwinNormal(
        {required StructWithZeroFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .funcStructWithZeroFieldTwinNormal(arg: arg, hint: hint);

Future<TupleStructWithOneFieldTwinNormal> funcTupleStructWithOneFieldTwinNormal(
        {required TupleStructWithOneFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithOneFieldTwinNormal(arg: arg, hint: hint);

Future<TupleStructWithTwoFieldTwinNormal> funcTupleStructWithTwoFieldTwinNormal(
        {required TupleStructWithTwoFieldTwinNormal arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithTwoFieldTwinNormal(arg: arg, hint: hint);

class StructWithOneFieldTwinNormal {
  final int a;

  const StructWithOneFieldTwinNormal({
    required this.a,
  });
}

class StructWithTwoFieldTwinNormal {
  final int a;
  final int b;

  const StructWithTwoFieldTwinNormal({
    required this.a,
    required this.b,
  });
}

class StructWithZeroFieldTwinNormal {
  const StructWithZeroFieldTwinNormal();
}

class TupleStructWithOneFieldTwinNormal {
  final int field0;

  const TupleStructWithOneFieldTwinNormal({
    required this.field0,
  });
}

class TupleStructWithTwoFieldTwinNormal {
  final int field0;
  final int field1;

  const TupleStructWithTwoFieldTwinNormal({
    required this.field0,
    required this.field1,
  });
}
