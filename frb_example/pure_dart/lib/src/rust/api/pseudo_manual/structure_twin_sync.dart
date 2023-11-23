// ignore_for_file: invalid_use_of_internal_member

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

StructWithOneFieldTwinSync funcStructWithOneFieldTwinSync(
        {required StructWithOneFieldTwinSync arg, dynamic hint}) =>
    RustLib.instance.api.funcStructWithOneFieldTwinSync(arg: arg, hint: hint);

StructWithTwoFieldTwinSync funcStructWithTwoFieldTwinSync(
        {required StructWithTwoFieldTwinSync arg, dynamic hint}) =>
    RustLib.instance.api.funcStructWithTwoFieldTwinSync(arg: arg, hint: hint);

StructWithZeroFieldTwinSync funcStructWithZeroFieldTwinSync(
        {required StructWithZeroFieldTwinSync arg, dynamic hint}) =>
    RustLib.instance.api.funcStructWithZeroFieldTwinSync(arg: arg, hint: hint);

TupleStructWithOneFieldTwinSync funcTupleStructWithOneFieldTwinSync(
        {required TupleStructWithOneFieldTwinSync arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithOneFieldTwinSync(arg: arg, hint: hint);

TupleStructWithTwoFieldTwinSync funcTupleStructWithTwoFieldTwinSync(
        {required TupleStructWithTwoFieldTwinSync arg, dynamic hint}) =>
    RustLib.instance.api
        .funcTupleStructWithTwoFieldTwinSync(arg: arg, hint: hint);

class StructWithOneFieldTwinSync {
  final int a;

  const StructWithOneFieldTwinSync({
    required this.a,
  });
}

class StructWithTwoFieldTwinSync {
  final int a;
  final int b;

  const StructWithTwoFieldTwinSync({
    required this.a,
    required this.b,
  });
}

class StructWithZeroFieldTwinSync {
  const StructWithZeroFieldTwinSync();
}

class TupleStructWithOneFieldTwinSync {
  final int field0;

  const TupleStructWithOneFieldTwinSync({
    required this.field0,
  });
}

class TupleStructWithTwoFieldTwinSync {
  final int field0;
  final int field1;

  const TupleStructWithTwoFieldTwinSync({
    required this.field0,
    required this.field1,
  });
}
