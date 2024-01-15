// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `optional_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/optional_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handleOptionalReturn', () async {
    expect((await handleOptionalReturnTwinSync(left: 1, right: 1))!, 1);
    expect(await handleOptionalReturnTwinSync(left: 2, right: 0), null);
  });

  test('dart call handleOptionalStruct', () async {
    {
      expect(await handleOptionalStructTwinSync(), null);
    }

    {
      final message = 'Hello there.';
      final ret = await handleOptionalStructTwinSync(document: message);
      if (ret == null) {
        fail('handleOptionalStruct returned null for non-null document');
      }
      expect(ret.tag, 'div');
      expect(ret.text, null);
      expect(ret.attributes?[0].key, 'id');
      expect(ret.attributes?[0].value, 'root');

      expect(ret.children?[0].tag, 'p');
      expect(ret.children?[0].text, null);
      expect(ret.children?[0].attributes, null);
      expect(ret.children?[0].children?[0].text, message);
    }
  });

  test('dart call handleOptionalIncrement', () async {
    expect(await handleOptionalIncrementTwinSync(), null);
    {
      var ret = await handleOptionalIncrementTwinSync(
          opt: ExoticOptionalsTwinSync(attributesNullable: []));
      if (ret == null) fail('increment returned null for non-null params');
      final loopFor = 20;
      for (var i = 1; i < loopFor; i++) {
        ret = await handleOptionalIncrementTwinSync(opt: ret);
      }
      if (ret == null) fail('ret nulled after loop');
      expect(ret.int32, loopFor, reason: 'int32');
      expect(ret.int64?.toInt(), loopFor, reason: 'int64');
      expect(ret.float64, loopFor, reason: 'float64');
      expect(ret.boolean, false);
      expect(ret.zerocopy?.length, loopFor);
      expect(ret.int8List?.length, loopFor);
      expect(ret.uint8List?.length, loopFor);
      expect(ret.attributesNullable, List.filled(loopFor, null));
      expect(ret.nullableAttributes, List.filled(loopFor, null));
      expect(ret.newtypeint?.field0.toInt(), loopFor, reason: 'NewTypeInt');
    }
  });

  test('dart call handleIncrementBoxedOptional', () async {
    {
      expect(await handleIncrementBoxedOptionalTwinSync(), 42);
    }

    {
      var ret = 0.0;
      final loopFor = 100;
      for (var i = 0; i < loopFor; i++) {
        ret = await handleIncrementBoxedOptionalTwinSync(opt: ret);
      }
      expect(ret, loopFor);
    }
  });

  test('dart call handleOptionBoxArguments', () async {
    print(await handleOptionBoxArgumentsTwinSync());

    {
      final optional10 = await handleOptionBoxArgumentsTwinSync(
        boolbox: true,
        structbox: await handleOptionalIncrementTwinSync(
            opt: ExoticOptionalsTwinSync(attributesNullable: [])),
      );
      print(optional10);
    }
  });
}
