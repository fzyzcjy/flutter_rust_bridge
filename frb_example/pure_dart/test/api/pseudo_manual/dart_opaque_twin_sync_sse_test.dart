// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () async {
    await futurizeVoidTwinSyncSse(loopBackArrayGetTwinSyncSse(
        opaque: await loopBackArrayTwinSyncSse(opaque: f)));
    await futurizeVoidTwinSyncSse(loopBackVecGetTwinSyncSse(
        opaque: await loopBackVecTwinSyncSse(opaque: f)));
    await futurizeVoidTwinSyncSse(loopBackOptionGetTwinSyncSse(
        opaque: await loopBackOptionTwinSyncSse(opaque: f)));

    var back1 = await loopBackTwinSyncSse(opaque: f) as String Function();
    expect(back1(), 'Test_String');
    var back2 = await loopBackTwinSyncSse(opaque: back1) as String Function();
    expect(back2(), 'Test_String');
    if (!kIsWeb) expect(identical(back2, f), isTrue);
  });

  test('drop', () async {
    expect(
        await asyncAcceptDartOpaqueTwinSyncSse(
            opaque: createLargeList(mb: 200)),
        'async test');
  });

  test('nested', () async {
    var str = await createNestedDartOpaqueTwinSyncSse(opaque1: f, opaque2: f);
    await futurizeVoidTwinSyncSse(getNestedDartOpaqueTwinSyncSse(opaque: str));
  });

  test('enum', () async {
    var en = await createEnumDartOpaqueTwinSyncSse(opaque: f);
    await futurizeVoidTwinSyncSse(getEnumDartOpaqueTwinSyncSse(opaque: en));
  });

  test('nested', () async {
    var str = await createNestedDartOpaqueTwinSyncSse(opaque1: f, opaque2: f);
    await futurizeVoidTwinSyncSse(getNestedDartOpaqueTwinSyncSse(opaque: str));
  });

  test('enum', () async {
    var en = await createEnumDartOpaqueTwinSyncSse(opaque: f);
    await futurizeVoidTwinSyncSse(getEnumDartOpaqueTwinSyncSse(opaque: en));
  });

  test('clone DartOpaque at rust side', () async {
    final opaque = (int a) => a + 1;
    final output = await cloneDartOpaqueTwinSyncSse(opaque: opaque);
    expect(output.length, 10);
    for (final x in output) {
      if (!kIsWeb) expect(identical(x, opaque), true);
      expect((x as Function)(42), 42 + 1);
    }
  });
}
