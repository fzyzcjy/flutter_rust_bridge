// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/misc_no_twin_example_a.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('ItemContainerSolutionOneTwinNormal', () async {
    final container =
        await ItemContainerSolutionOneTwinNormal.createTwinNormal();

    expect(await container.getItemContentsTwinNormal(), [100]);

    expect(container.name, 'hi');
    container.name = 'hello';
    expect(container.name, 'hello');
  });

  test('ItemContainerSolutionTwoTwinNormal', () async {
    final container =
        await ItemContainerSolutionTwoTwinNormal.createTwinNormal();

    expect(await container.getItemContentsTwinNormal(), [100]);

    expect(container.name, 'hi');
    container.name = 'hello';
    expect(container.name, 'hello');
  });

  test('getter and setter', () async {
    final obj = StructWithSimpleSetterTwinNormal();

    obj.simpleSetter = 42;
    expect(obj.simpleGetter, 42);

    obj.something = 200;
    expect(obj.something, 200);
  });

  test('MyStructWithTryFromTwinNormal', () async {
    final object = await MyStructWithTryFromTwinNormal.tryFrom(value: 'hello');
    expect(await object.valueTwinNormal(), 'hello');
  });

  test('feature test enabled', () async {
    expect(await featureGatedFunction(), "test");
  });

  test('const int', () async {
    expect(constIntTwinNormal, 42);
  });
  test('const array', () async {
    expect(constArrayTwinNormal.inner, [1.5, 3.0, 6.0]);
  });

  test('MyEnumWithJsonSerializableTwinNormal', () async {
    final dict = {'runtimeType': 'apple', 'field0': 'hi'};
    final obj = MyEnumWithJsonSerializableTwinNormal.fromJson(dict);
    expect(obj.toJson(), dict);
  });

  test('MyStructWithJsonSerializableTwinNormal', () async {
    final dict = {'fieldOne': 'hi'};
    final obj = MyStructWithJsonSerializableTwinNormal.fromJson(dict);
    expect(obj.toJson(), dict);
  });
}
