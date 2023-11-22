// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<StructWithZeroField> funcStructWithZeroFieldTwinNormal(
        {required StructWithZeroField arg, dynamic hint}) =>
    RustLib.instance.api
        .funcStructWithZeroFieldTwinNormal(arg: arg, hint: hint);

class StructWithZeroField {
  const StructWithZeroField();
}
