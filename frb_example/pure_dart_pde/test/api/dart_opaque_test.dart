// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/dart_opaque.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () async {
    await futurizeVoidTwinNormal(loopBackArrayGetTwinNormal(
        opaque: await loopBackArrayTwinNormal(opaque: f)));
    await futurizeVoidTwinNormal(loopBackVecGetTwinNormal(
        opaque: await loopBackVecTwinNormal(opaque: f)));
    await futurizeVoidTwinNormal(loopBackOptionGetTwinNormal(
        opaque: await loopBackOptionTwinNormal(opaque: f)));

    var back1 = await loopBackTwinNormal(opaque: f) as String Function();
    expect(back1(), 'Test_String');
    var back2 = await loopBackTwinNormal(opaque: back1) as String Function();
    expect(back2(), 'Test_String');
    if (!kIsWeb) expect(identical(back2, f), isTrue);
  });

  test('drop', () async {
    expect(
        await asyncAcceptDartOpaqueTwinNormal(opaque: createLargeList(mb: 200)),
        'async test');
  });

  test('nested', () async {
    var str = await createNestedDartOpaqueTwinNormal(opaque1: f, opaque2: f);
    await futurizeVoidTwinNormal(getNestedDartOpaqueTwinNormal(opaque: str));
  });

  test('enum', () async {
    var en = await createEnumDartOpaqueTwinNormal(opaque: f);
    await futurizeVoidTwinNormal(getEnumDartOpaqueTwinNormal(opaque: en));
  });

  test('nested', () async {
    var str = await createNestedDartOpaqueTwinNormal(opaque1: f, opaque2: f);
    await futurizeVoidTwinNormal(getNestedDartOpaqueTwinNormal(opaque: str));
  });

  test('enum', () async {
    var en = await createEnumDartOpaqueTwinNormal(opaque: f);
    await futurizeVoidTwinNormal(getEnumDartOpaqueTwinNormal(opaque: en));
  });

  test('clone DartOpaque at rust side', () async {
    final opaque = (int a) => a + 1;
    final output = await cloneDartOpaqueTwinNormal(opaque: opaque);
    expect(output.length, 10);
    for (final x in output) {
      if (!kIsWeb) expect(identical(x, opaque), true);
      expect((x as Function)(42), 42 + 1);
    }
  });
}
