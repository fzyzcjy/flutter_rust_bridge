// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Stream<String> funcStreamRealisticTwinNormal(
        {required String arg, dynamic hint}) =>
    RustLib.instance.api.funcStreamRealisticTwinNormal(arg: arg, hint: hint);

Stream<String> funcStreamReturnErrorTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcStreamReturnErrorTwinNormal(hint: hint);

Stream<String> funcStreamReturnPanicTwinNormal({dynamic hint}) =>
    RustLib.instance.api.funcStreamReturnPanicTwinNormal(hint: hint);

Stream<int> funcStreamSinkArgPositionTwinNormal(
        {required int a, required int b, dynamic hint}) =>
    RustLib.instance.api
        .funcStreamSinkArgPositionTwinNormal(a: a, b: b, hint: hint);
