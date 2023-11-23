// ignore_for_file: invalid_use_of_internal_member, unused_import

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

Stream<MyStreamEntry> handleStreamOfStruct({dynamic hint}) =>
    RustLib.instance.api.handleStreamOfStruct(hint: hint);

class MyStreamEntry {
  final String hello;

  const MyStreamEntry({
    required this.hello,
  });

  @override
  int get hashCode => hello.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyStreamEntry &&
          runtimeType == other.runtimeType &&
          hello == other.hello;
}
