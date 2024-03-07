// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/constructor.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('constructor', () {
    group('ConstructorTranslatableStructTwinNormal', () {
      test('call Rust constructor', () async {
        expect(
            (await ConstructorTranslatableStructTwinNormal.newInstance()).one,
            'hello');
      });

      test('call Dart native constructor', () async {
        expect(ConstructorTranslatableStructTwinNormal(one: 'a').one, 'a');
      });
    });

    group('ConstructorOpaqueStructTwinNormal', () {
      test('call Rust constructor', () async {
        final object = await ConstructorOpaqueStructTwinNormal.newInstance();
        object.check();
      });
    });

    group('ConstructorTranslatableSyncStructTwinNormal', () {
      test('call Rust constructor', () async {
        expect(ConstructorTranslatableSyncStructTwinNormal().one, 'hello');
      });

      test('call Dart native constructor', () async {
        expect(
            ConstructorTranslatableSyncStructTwinNormal.raw(one: 'a').one, 'a');
      });
    });

    group('ConstructorOpaqueSyncStructTwinNormal', () {
      test('call Rust constructor', () async {
        final object = ConstructorOpaqueSyncStructTwinNormal();
        object.check();
      });
    });
  });
}
