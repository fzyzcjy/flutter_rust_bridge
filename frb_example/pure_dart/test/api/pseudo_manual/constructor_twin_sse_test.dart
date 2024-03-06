// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `constructor_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/constructor_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('constructor', () {
    group('ConstructorTranslatableStructTwinSse', () {
      test('call Rust constructor', () async {
        expect((await ConstructorTranslatableStructTwinSse.newInstance()).one,
            'hello');
      });

      test('call Dart native constructor', () async {
        expect(ConstructorTranslatableStructTwinSse(one: 'a').one, 'a');
      });
    });

    group('ConstructorOpaqueStructTwinSse', () {
      test('call Rust constructor', () async {
        final object = await ConstructorOpaqueStructTwinSse.newInstance();
        object.check();
      });
    });

    group('ConstructorTranslatableSyncStructTwinSse', () {
      test('call Rust constructor', () async {
        expect(ConstructorTranslatableSyncStructTwinSse().one, 'hello');
      });

      test('call Dart native constructor', () async {
        expect(ConstructorTranslatableSyncStructTwinSse.raw(one: 'a').one, 'a');
      });
    });

    group('ConstructorOpaqueSyncStructTwinSse', () {
      test('call Rust constructor', () async {
        final object = ConstructorOpaqueSyncStructTwinSse();
        object.check();
      });
    });
  });
}
