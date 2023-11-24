import 'package:frb_example_pure_dart/src/rust/api/optional.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call handleOptionalReturn', () async {
    expect((await handleOptionalReturn(left: 1, right: 1))!, 1);
    expect(await handleOptionalReturn(left: 2, right: 0), null);
  });

  test('dart call handleOptionalStruct', () async {
    {
      expect(await handleOptionalStruct(), null);
    }

    {
      final message = 'Hello there.';
      final ret = await handleOptionalStruct(document: message);
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
    expect(await handleOptionalIncrement(), null);
    {
      var ret = await handleOptionalIncrement(opt: ExoticOptionals(attributesNullable: []));
      if (ret == null) fail('increment returned null for non-null params');
      final loopFor = 20;
      for (var i = 1; i < loopFor; i++) {
        ret = await handleOptionalIncrement(opt: ret);
      }
      if (ret == null) fail('ret nulled after loop');
      expect(ret.int32, loopFor, reason: 'int32');
      expect(ret.int64.toInt(), loopFor, reason: 'int64');
      expect(ret.float64, loopFor, reason: 'float64');
      expect(ret.boolean, false);
      expect(ret.zerocopy?.length, loopFor);
      expect(ret.int8List?.length, loopFor);
      expect(ret.uint8List?.length, loopFor);
      expect(ret.attributesNullable, List.filled(loopFor, null));
      expect(ret.nullableAttributes, List.filled(loopFor, null));
      expect(ret.newtypeint?.field0, loopFor, reason: 'NewTypeInt');
    }
  });

  test('dart call handleIncrementBoxedOptional', () async {
    {
      expect(await handleIncrementBoxedOptional(), 42);
    }

    {
      var ret = 0.0;
      final loopFor = 100;
      for (var i = 0; i < loopFor; i++) {
        ret = await handleIncrementBoxedOptional(opt: ret);
      }
      expect(ret, loopFor);
    }
  });

  test('dart call handleOptionBoxArguments', () async {
    print(await handleOptionBoxArguments());

    {
      final optional10 = await handleOptionBoxArguments(
        boolbox: true,
        structbox: await handleOptionalIncrement(opt: ExoticOptionals(attributesNullable: [])),
      );
      print(optional10);
    }
  });
}
