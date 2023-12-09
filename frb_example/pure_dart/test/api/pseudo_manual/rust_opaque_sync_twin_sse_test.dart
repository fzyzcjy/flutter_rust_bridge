// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_sync_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_sync_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create', () {
    var data = syncCreateOpaqueTwinSse();
    data.dispose();
  });

  test('option', () async {
    var x = syncOptionRustOpaqueTwinSse();
    expect(x, isNotNull);
    x!.dispose();
  });

  test('nonclone', () async {
    var data = syncCreateNonCloneTwinSse();
    var data2 = await runNonCloneTwinSse(clone: data);
    expect(data2, "content");
    data.dispose();
  });

  test('double call', () {
    var data = syncCreateSyncOpaqueTwinSse();
    expect(
        syncRunOpaqueTwinSse(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        syncRunOpaqueTwinSse(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
  });

  test('call after drop', () async {
    var data = syncCreateSyncOpaqueTwinSse();
    expect(
        syncRunOpaqueTwinSse(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectLater(() => syncRunOpaqueTwinSse(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('check generator', () {
    expect(frbSyncGeneratorTestTwinSse().runtimeType == FrbOpaqueSyncReturn,
        isTrue);
  });
}
