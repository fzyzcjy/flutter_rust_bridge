// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/primitive_misc.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

// NOTE majority of tests are in `pseudo_manual/*`
Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call primitiveTypes', () async {
    expect(
        await primitiveTypesTwinNormal(
            myI32: 123,
            myI64: PlatformInt64Util.from(10000000000000),
            myF64: 12345678901234567890.123,
            myBool: true),
        42);
  });

  test('dart call primitiveU32', () async {
    expect(await primitiveU32TwinNormal(myU32: 0xff112233), 0xfe112233);
  });
}
