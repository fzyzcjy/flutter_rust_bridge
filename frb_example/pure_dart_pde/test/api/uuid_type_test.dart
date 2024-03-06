// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/uuid_type.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import 'package:uuid/uuid.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('Uuid', () async {
    final uuid = Uuid();
    final id = uuid.v4obj();
    final output = await handleUuidTwinNormal(id: id);
    expect(id, output);
  });

  test('Vec<Uuid>', () async {
    final uuid = Uuid();
    final ids =
        List<UuidValue>.from([uuid.v4obj(), uuid.v1obj(), uuid.v4obj()]);
    final outputs = await handleUuidsTwinNormal(ids: ids);
    expect(ids, outputs);
  });

  test('nested uuid types', () async {
    final uuid = Uuid();
    final id = uuid.v4obj();
    // final ids =
    //     List<UuidValue>.from([uuid.v4obj(), uuid.v1obj(), uuid.v4obj()]);
    final wrapper = FeatureUuidTwinNormal(one: id);
    final outputs = await handleNestedUuidsTwinNormal(ids: wrapper);
    expect(wrapper.one, outputs.one);
    // expect(wrapper.many, outputs.many);
  });
}
