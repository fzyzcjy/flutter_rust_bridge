// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<int> funcReturnErrorTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcReturnErrorTwinNormal(hint: hint);

Future<int> funcReturnPanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcReturnPanicTwinNormal(hint: hint);
