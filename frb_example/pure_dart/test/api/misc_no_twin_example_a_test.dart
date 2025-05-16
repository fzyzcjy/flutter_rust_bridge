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

  // The `TypeForIgnore` type has a single field called `field_1` which the
  // constructor always sets to 0.  The field is marked as `#[frb(ignore)]`
  // which means that the FRB code generater should not generate automatic
  // accessors for it.
  //
  // To ensure that no accessors were generated the type also has a `field_1`
  // method which always returns 1.
  //
  // - If the attribute isn't recognized then FRB codegen will generate an
  //   automatic accessor and we'll get a compile error due to the `field_1`
  //   accessor conflict.
  // - If the the automatic accessor ends up taking precedence then the
  //   following will fail to compile because the accessor is called as a
  //   function.
  test('make sure #[frb(ignore)] works', () async {
    var t = await TypeForIgnore.newInstance();
    expect(await t.field1(), 1);
  });

  // `#[frb(ignore_all)]` disables all auto accessors for the type by default
  // so `field_1` should be overridden as in the above test case.
  //
  // `field_2` is marked with `#[frb(unignore)]` so its auto accessor should still be generated.
  test('make sure #[frb(ignore_all)] works', () async {
    var t = await TypeForIgnoreAll.newInstance();
    expect(await t.field1(), 1);
    expect(await t.field2, 0);
  });
}
