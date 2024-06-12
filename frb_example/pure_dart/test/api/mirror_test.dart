// FRB_INTERNAL_GENERATOR: {"addCode":"import 'package:frb_example_pure_dart/src/rust/api/mirror.dart';"}

// ignore_for_file: duplicate_import

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/mirror.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call getAppSettings()', () async {
    var settings = await getAppSettingsTwinNormal();
    expect(settings.version, "1.0.0-rc.1");
    expect(settings.mode, ApplicationMode.standalone);
    expect(settings.env.vars[0].field0, "myenv");
  });

  test('dart call isAppEmbedded()', () async {
    expect(
        await isAppEmbeddedTwinNormal(
            appSettings: ApplicationSettings(
                name: "from dart",
                version: "XX",
                mode: ApplicationMode.embedded,
                env: ApplicationEnv(vars: [
                  ApplicationEnvVar(field0: "sendback", field1: true)
                ]))),
        true);
  });

  test('dart call app_settings_stream', () async {
    final settings = await appSettingsStreamTwinNormal().first;
    _testAppSettings(settings);
  });

  test('dart call app_settings_vec_stream', () async {
    final settings = await appSettingsVecStreamTwinNormal().first;
    _testAppSettings(settings[0]);
    _testAppSettings(settings[1]);
  });

  test('dart call mirror_struct_stream', () async {
    final ret = await mirrorStructStreamTwinNormal().first;
    _testAppSettings(ret.a);
    expect(ret.b.content, true);
    expect(ret.c[0], MyEnum.true_);
    expect(ret.c[1], MyEnum.false_);
    _testAppSettings(ret.d[0]);
    _testAppSettings(ret.d[1]);
  });

  test('dart call mirror_tuple_stream', () async {
    final (settings, rawStringEnum) = await mirrorTupleStreamTwinNormal().first;
    _testAppSettings(settings);
    expect(rawStringEnum is RawStringEnumMirrored_Raw, true);
    expect((rawStringEnum as RawStringEnumMirrored_Raw).field0.value, "test");
  });

  test('dart call getMessage()', () async {
    var message = await getMessageTwinNormal();
    expect(message is ApplicationMessage_RenderPixel, true);
    message as ApplicationMessage_RenderPixel;
    expect(message.x, 5);
    expect(message.y, 10);

    _createGarbage();
    await Future.delayed(Duration(seconds: 1));
    _createGarbage();
    await Future.delayed(Duration(seconds: 1));
  });

  test('dart call repeatNumber()', () async {
    var numbers = await repeatNumberTwinNormal(num: 1, times: BigInt.from(10));
    expect(numbers.field0.toList(),
        Int32List.fromList([1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
  });

  test('dart call repeatSequence()', () async {
    var sequences =
        await repeatSequenceTwinNormal(seq: 1, times: BigInt.from(10));
    expect(sequences.field0.toList(),
        Int32List.fromList([1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
  });

  test('dart call firstNumber()', () async {
    var numbers = Numbers(field0: Int32List.fromList([1]));
    var first = await firstNumberTwinNormal(nums: numbers);
    expect(first, 1);
  });

  test('dart call firstSequence()', () async {
    var sequences = Sequences(field0: Int32List.fromList([1]));
    var first = await firstSequenceTwinNormal(seqs: sequences);
    expect(first, 1);
  });

  test('test mirrored raw structs', () async {
    final output = await testRawStringMirroredTwinNormal();
    expect(output, isA<RawStringMirrored>());
    expect(output.value, "test");
  });

  test('test nested mirror raw', () async {
    final output = await testNestedRawStringMirroredTwinNormal();
    expect(output, isA<NestedRawStringMirrored>());
    expect(output.raw, isA<RawStringMirrored>());
    expect(output.raw.value, "test");
  });

  test('test raw string enum', () async {
    final output1 = await testRawStringEnumMirroredTwinNormal(nested: true);
    expect(output1 is RawStringEnumMirrored_Nested, true);
    expect((output1 as RawStringEnumMirrored_Nested).field0.raw.value, "test");

    final output2 = await testRawStringEnumMirroredTwinNormal(nested: false);
    expect(output2 is RawStringEnumMirrored_Raw, true);
    expect((output2 as RawStringEnumMirrored_Raw).field0.value, "test");
  });

  test('test list of raw nested strings', () async {
    final output = await testListOfRawNestedStringMirroredTwinNormal();
    expect(output.raw.length, 1);
    expect(output.raw[0].raw.value, "test");
  });

  test('test fallible vec of raw string', () async {
    final output = await testFallibleOfRawStringMirroredTwinNormal();
    expect(output.length, 1);
    expect(output.first.value, "test");
  });

  test('test contains mirrored sub struct', () async {
    final output = await testContainsMirroredSubStructTwinNormal();
    expect(output, isA<ContainsMirroredSubStructTwinNormal>());
    expect(output.test, isA<RawStringMirrored>());
    expect(output.test.value, "test");
    expect(output.test2.a, "test");
  });

  test('test_hashmap_with_mirrored_value', () async {
    final output = await testHashmapWithMirroredValueTwinNormal();
    expect(output.map, {'key': HashMapValue(inner: 'value')});
  });

  test('mirror_enum_stream_twin_normal', () async {
    final output = await mirrorEnumStreamTwinNormal().toList();
    expect(
      output,
      orderedEquals([
        ApplicationMode.embedded,
        ApplicationMode.standalone,
      ]),
    );
  });

  test('mirror_option_enum_stream_twin_normal', () async {
    final output = await mirrorOptionEnumStreamTwinNormal().toList();
    expect(
      output,
      orderedEquals([
        ApplicationMode.embedded,
        null,
        ApplicationMode.standalone,
      ]),
    );
  });

  test('mirror_vec_enum_stream_twin_normal', () async {
    final output = await mirrorVecEnumStreamTwinNormal().toList();
    expect(output, [
      orderedEquals([ApplicationMode.embedded]),
      orderedEquals([ApplicationMode.standalone]),
    ]);
  });

  test('mirror_map_enum_stream_twin_normal', () async {
    final output = await mirrorMapEnumStreamTwinNormal().toList();
    expect(output, [
      allOf(
        containsPair(0, ApplicationMode.embedded),
        containsPair(1, ApplicationMode.standalone),
      )
    ]);
  });

  test('mirror_set_enum_stream_twin_normal', () async {
    final output = await mirrorSetEnumStreamTwinNormal().toList();
    expect(output, [
      unorderedEquals([
        ApplicationMode.embedded,
        ApplicationMode.standalone,
      ])
    ]);
  });

  test('mirror_array_enum_stream_twin_normal', () async {
    final output = await mirrorArrayEnumStreamTwinNormal().toList();
    expect(output, [
      orderedEquals([
        ApplicationMode.embedded,
        ApplicationMode.standalone,
      ])
    ]);
  });
}

int _createGarbage() {
  print('dart create garbage (thus make it more possible to GC)');
  var cum = 0;
  for (var i = 0; i < 1000; ++i) {
    final l = List.filled(5000, 42);
    cum += l[42];
  }
  return cum;
}

void _testAppSettings(ApplicationSettings settings) {
  expect(settings.version, "1.0.0-rc.1");
  expect(settings.mode, ApplicationMode.standalone);
  expect(settings.env.vars[0].field0, "myenv");
}
