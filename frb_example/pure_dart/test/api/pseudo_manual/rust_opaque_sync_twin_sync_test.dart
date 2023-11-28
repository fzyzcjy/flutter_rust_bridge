// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_sync_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_sync_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create', () {
    var data = syncCreateOpaqueTwinSync();
    data.dispose();
  });

  test('option', () async {
    var x = syncOptionRustOpaqueTwinSync();
    expect(x, isNotNull);
    x!.dispose();
  });

  test('nonclone', () async {
    var data = syncCreateNonCloneTwinSync();
    var data2 = await runNonCloneTwinSync(clone: data);
    expect(data2, "content");
    data.dispose();
  });

  test('double call', () {
    var data = syncCreateSyncOpaqueTwinSync();
    expect(
        syncRunOpaqueTwinSync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        syncRunOpaqueTwinSync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
  });

  test('call after drop', () {
    var data = syncCreateSyncOpaqueTwinSync();
    expect(
        syncRunOpaqueTwinSync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    expect(() => syncRunOpaqueTwinSync(opaque: data),
        throwsA(isA<PanicException>()));
  });

  test('check generator', () {
    expect(frbSyncGeneratorTestTwinSync().runtimeType == FrbOpaqueSyncReturn,
        isTrue);
  });
}
