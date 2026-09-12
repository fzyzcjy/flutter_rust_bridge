import 'dart:async';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  print('Action: Configure tests (end)');

  test('Web MoiArc release completes after pool contention', () async {
    reproduceMoiArcReleaseContention();
    final deadline = DateTime.now().add(const Duration(seconds: 5));
    while (!moiArcContentionValueWasDropped() && DateTime.now().isBefore(deadline)) {
      await Future<void>.delayed(const Duration(milliseconds: 10));
    }
    expect(moiArcContentionValueWasDropped(), isTrue);
  }, skip: !const bool.fromEnvironment('dart.library.js_interop'));
}
