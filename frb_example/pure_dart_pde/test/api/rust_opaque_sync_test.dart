// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse", "sync moi", "rustAsync moi", "sync sse moi", "rustAsync sse moi"], "enableAll": true}

import 'package:frb_example_pure_dart_pde/src/rust/api/rust_opaque.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/rust_opaque_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create', () {
    var data = syncCreateOpaqueTwinNormal();
    data.dispose();
  });

  test('option', () async {
    var x = syncOptionRustOpaqueTwinNormal();
    expect(x, isNotNull);
    x!.dispose();
  });

  test('nonclone', () async {
    var data = syncCreateNonCloneTwinNormal();
    var data2 = await runNonCloneTwinNormal(clone: data);
    expect(data2, "content");
    data.dispose();
  });

  // test('check generator', () {
  //   expect(
  //       frbSyncGeneratorTestTwinNormal().runtimeType ==
  //           FrbOpaqueSyncReturnTwinNormal,
  //       isTrue);
  // });
}
