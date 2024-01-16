// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// ignore_for_file: invalid_use_of_internal_member

import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  // Please ensure this is the only one test, since we usually do not init/dispose twice
  test('init and dispose', () async {
    expect(RustLib.instance.initialized, false);
    expect(() => RustLib.instance.api, throwsA(isA<StateError>()));

    await RustLib.init();

    expect(RustLib.instance.initialized, true);
    expect(RustLib.instance.api, isNot(isNull));

    expect(() => RustLib.init(), throwsA(isA<StateError>()));

    RustLib.dispose();
  });
}
