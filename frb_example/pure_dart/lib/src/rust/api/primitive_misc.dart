// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<int> primitiveTypes(
        {required int myI32,
        required BigInt myI64,
        required double myF64,
        required bool myBool,
        dynamic hint}) =>
    RustLib.instance.api.primitiveTypes(
        myI32: myI32, myI64: myI64, myF64: myF64, myBool: myBool, hint: hint);

Future<int> primitiveU32({required int myU32, dynamic hint}) =>
    RustLib.instance.api.primitiveU32(myU32: myU32, hint: hint);
