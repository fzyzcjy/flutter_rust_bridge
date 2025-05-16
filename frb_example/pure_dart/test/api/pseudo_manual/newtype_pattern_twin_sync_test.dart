// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `newtype_pattern_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/newtype_pattern_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handleNewtype', () async {
    final newtypeResp = await handleNewtypeTwinSync(
        arg: NewTypeIntTwinSync(field0: PlatformInt64Util.from(42)));
    expect(newtypeResp.field0.toInt(), 84);
  });
}
