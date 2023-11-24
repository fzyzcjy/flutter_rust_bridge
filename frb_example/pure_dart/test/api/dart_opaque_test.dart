import 'package:frb_example_pure_dart/src/rust/api/dart_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  String f() => 'Test_String';

  test('loopback', () async {
    await loopBackArrayGet(opaque: await loopBackArray(opaque: f));
    await loopBackVecGet(opaque: await loopBackVec(opaque: f));
    await loopBackOptionGet(opaque: await loopBackOption(opaque: f));

    var back1 = await loopBack(opaque: f) as String Function();
    expect(back1(), 'Test_String');
    var back2 = await loopBack(opaque: back1) as String Function();
    expect(back2(), 'Test_String');
    expect(identical(back2, f), isTrue);
  });

  test('drop', () async {
    expect(await asyncAcceptDartOpaque(opaque: createLargeList(mb: 200)), 'async test');
  });

  test('nested', () async {
    var str = await createNestedDartOpaque(opaque1: f, opaque2: f);
    await getNestedDartOpaque(opaque: str);
  });

  test('enum', () async {
    var en = await createEnumDartOpaque(opaque: f);
    await getEnumDartOpaque(opaque: en);
  });

  test('nested', () async {
    var str = await createNestedDartOpaque(opaque1: f, opaque2: f);
    await getNestedDartOpaque(opaque: str);
  });

  test('enum', () async {
    var en = await createEnumDartOpaque(opaque: f);
    await getEnumDartOpaque(opaque: en);
  });
}
