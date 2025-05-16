// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/optional_primitive_misc.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call optional primitiveTypes', () async {
    expect(
        await primitiveOptionalTypesTwinNormal(
            myI32: null, myI64: null, myF64: null, myBool: null),
        0);
    expect(
        await primitiveOptionalTypesTwinNormal(
            myI32: 0,
            myI64: PlatformInt64Util.from(0),
            myF64: 0,
            myBool: false),
        4);
    expect(
        await primitiveOptionalTypesTwinNormal(
            myI32: 123,
            myI64: PlatformInt64Util.from(123),
            myF64: 123,
            myBool: true),
        4);
  });
}
