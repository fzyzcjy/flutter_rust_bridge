// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_sync_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse", "sync moi", "rustAsync moi", "sync sse moi", "rustAsync sse moi"], "enableAll": true}

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_sse_moi.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_sync_twin_sse_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create', () {
    var data = syncCreateOpaqueTwinSseMoi();
    data.dispose();
  });

  test('option', () async {
    var x = syncOptionRustOpaqueTwinSseMoi();
    expect(x, isNotNull);
    x!.dispose();
  });

  test('nonclone', () async {
    var data = syncCreateNonCloneTwinSseMoi();
    var data2 = await runNonCloneTwinSseMoi(clone: data);
    expect(data2, "content");
    data.dispose();
  });

  // test('check generator', () {
  //   expect(
  //       frbSyncGeneratorTestTwinSseMoi().runtimeType ==
  //           FrbOpaqueSyncReturnTwinSseMoi,
  //       isTrue);
  // });
}
