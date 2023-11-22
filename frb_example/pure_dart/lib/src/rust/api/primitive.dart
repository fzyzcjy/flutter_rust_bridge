// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';

Future<bool> examplePrimitiveTypeBool({required bool arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeBool(arg: arg, hint: hint);

Future<double> examplePrimitiveTypeF32({required double arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeF32(arg: arg, hint: hint);

Future<double> examplePrimitiveTypeF64({required double arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeF64(arg: arg, hint: hint);

Future<int> examplePrimitiveTypeI16({required int arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeI16(arg: arg, hint: hint);

Future<int> examplePrimitiveTypeI32({required int arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeI32(arg: arg, hint: hint);

Future<BigInt> examplePrimitiveTypeI64({required BigInt arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeI64(arg: arg, hint: hint);

Future<int> examplePrimitiveTypeI8({required int arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeI8(arg: arg, hint: hint);

Future<int> examplePrimitiveTypeU16({required int arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeU16(arg: arg, hint: hint);

Future<int> examplePrimitiveTypeU32({required int arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeU32(arg: arg, hint: hint);

Future<BigInt> examplePrimitiveTypeU64({required BigInt arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeU64(arg: arg, hint: hint);

Future<int> examplePrimitiveTypeU8({required int arg, dynamic hint}) =>
    RustLib.instance.api.examplePrimitiveTypeU8(arg: arg, hint: hint);
