// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<String> funcStringTwinNormal({required String arg, dynamic hint}) =>
    RustLib.instance.api.funcStringTwinNormal(arg: arg, hint: hint);

Future<void> funcReturnUnitTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcReturnUnitTwinNormal(hint: hint);

Future<List<MySize>> handleListOfStruct(
        {required List<MySize> l, dynamic hint}) =>
    RustLib.instance.api.handleListOfStruct(l: l, hint: hint);

Future<List<String>> handleStringList(
        {required List<String> names, dynamic hint}) =>
    RustLib.instance.api.handleStringList(names: names, hint: hint);
