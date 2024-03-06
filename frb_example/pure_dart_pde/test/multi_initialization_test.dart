// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/simple_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  test('Can initialize twice (mimic Dart hot restart)', () async {
    await RustLib.init();

    expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
    expect(simpleAdderTwinSync(a: 42, b: 100), 142);

    // ignore: invalid_use_of_internal_member
    RustLib.instance.resetState();

    await RustLib.init();

    expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
    expect(simpleAdderTwinSync(a: 42, b: 100), 142);
  });
}
