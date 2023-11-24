// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<int?> primitiveOptionalTypes(
        {int? myI32,
        BigInt? myI64,
        double? myF64,
        bool? myBool,
        dynamic hint}) =>
    RustLib.instance.api.primitiveOptionalTypes(
        myI32: myI32, myI64: myI64, myF64: myF64, myBool: myBool, hint: hint);
