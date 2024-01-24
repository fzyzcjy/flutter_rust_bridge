// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:isolate';

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/simple_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('can execute in another isolate', () async {
    // Can call in current isolate
    expect(await simpleAdderTwinNormal(a: 42, b: 100), 142);
    expect(simpleAdderTwinSync(a: 42, b: 100), 142);

    final isolateResults = await Isolate.run(() async {
      await RustLib.init();

      // Can call in new isolate
      final ans = (
        await simpleAdderTwinNormal(a: 42, b: 100),
        simpleAdderTwinSync(a: 42, b: 100),
      );

      RustLib.dispose();

      return ans;
    });
    expect(isolateResults, (142, 142));
  });
}
