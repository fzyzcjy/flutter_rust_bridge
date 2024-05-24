// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `optional_primitive_misc_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/optional_primitive_misc_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call optional primitiveTypes', () async {
    expect(
        await primitiveOptionalTypesTwinRustAsyncSse(
            myI32: null, myI64: null, myF64: null, myBool: null),
        0);
    expect(
        await primitiveOptionalTypesTwinRustAsyncSse(
            myI32: 0,
            myI64: PlatformInt64Util.from(0),
            myF64: 0,
            myBool: false),
        4);
    expect(
        await primitiveOptionalTypesTwinRustAsyncSse(
            myI32: 123,
            myI64: PlatformInt64Util.from(123),
            myF64: 123,
            myBool: true),
        4);
  });
}
