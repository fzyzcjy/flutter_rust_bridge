// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<MacroStruct> funcMacroStruct({required MacroStruct arg, dynamic hint}) =>
    RustLib.instance.api.funcMacroStruct(arg: arg, hint: hint);

class MacroStruct {
  final int data;

  const MacroStruct({
    required this.data,
  });

  @override
  int get hashCode => data.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MacroStruct &&
          runtimeType == other.runtimeType &&
          data == other.data;
}
