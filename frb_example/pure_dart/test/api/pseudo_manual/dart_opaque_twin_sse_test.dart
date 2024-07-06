// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_opaque_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () async {
    await futurizeVoidTwinSse(
        loopBackArrayGetTwinSse(opaque: await loopBackArrayTwinSse(opaque: f)));
    await futurizeVoidTwinSse(
        loopBackVecGetTwinSse(opaque: await loopBackVecTwinSse(opaque: f)));
    await futurizeVoidTwinSse(loopBackOptionGetTwinSse(
        opaque: await loopBackOptionTwinSse(opaque: f)));

    var back1 = await loopBackTwinSse(opaque: f) as String Function();
    expect(back1(), 'Test_String');
    var back2 = await loopBackTwinSse(opaque: back1) as String Function();
    expect(back2(), 'Test_String');
    if (!kIsWeb) expect(identical(back2, f), isTrue);
  });

  test('drop', () async {
    expect(await asyncAcceptDartOpaqueTwinSse(opaque: createLargeList(mb: 200)),
        'async test');
  });

  test('nested', () async {
    var str = await createNestedDartOpaqueTwinSse(opaque1: f, opaque2: f);
    await futurizeVoidTwinSse(getNestedDartOpaqueTwinSse(opaque: str));
  });

  test('enum', () async {
    var en = await createEnumDartOpaqueTwinSse(opaque: f);
    await futurizeVoidTwinSse(getEnumDartOpaqueTwinSse(opaque: en));
  });

  test('nested', () async {
    var str = await createNestedDartOpaqueTwinSse(opaque1: f, opaque2: f);
    await futurizeVoidTwinSse(getNestedDartOpaqueTwinSse(opaque: str));
  });

  test('enum', () async {
    var en = await createEnumDartOpaqueTwinSse(opaque: f);
    await futurizeVoidTwinSse(getEnumDartOpaqueTwinSse(opaque: en));
  });

  test('clone DartOpaque at rust side', () async {
    final opaque = (int a) => a + 1;
    final output = await cloneDartOpaqueTwinSse(opaque: opaque);
    expect(output.length, 10);
    for (final x in output) {
      if (!kIsWeb) expect(identical(x, opaque), true);
      expect((x as Function)(42), 42 + 1);
    }
  });
}
