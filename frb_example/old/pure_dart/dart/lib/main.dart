import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';
import 'package:uuid/uuid.dart';

import 'bridge_definitions.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';

const isWeb = bool.fromEnvironment('dart.library.html');

String? skipWeb([String reason = 'unspecified']) => isWeb ? 'Skipped on web (reason: $reason)' : null;

void main(List<String> args) async {
  String dylibPath = args[0];
  var releaseMode = true;
  assert(() {
    releaseMode = false;
    return true;
  }());
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final api = initializeExternalLibrary(dylibPath);
  tearDownAll(() => dispose());

  test('dart call primitiveTypes', () async {
    expect(await primitiveTypes(myI32: 123, myI64: 10000000000000, myF64: 12345678901234567890.123, myBool: true), 42);
  });

  test('dart call optional primitiveTypes', () async {
    expect(await primitiveOptionalTypes(myI32: null, myI64: null, myF64: null, myBool: null), 0);
    expect(await primitiveOptionalTypes(myI32: 0, myI64: 0, myF64: 0, myBool: false), 4);
    expect(await primitiveOptionalTypes(myI32: 123, myI64: 123, myF64: 123, myBool: true), 4);
  });

  test('dart call primitiveTypesSync', () {
    expect(primitiveTypesSync(myI32: 123, myI64: 10000000000000, myF64: 12345678901234567890.123, myBool: true), 42);
  });

  test('dart call primitiveU32', () async {
    expect(await primitiveU32(myU32: 0xff112233), 0xfe112233);
  });
  test('dart call primitiveU32Sync', () {
    expect(primitiveU32Sync(myU32: 0xff112233), 0xfe112233);
  });

  test('dart call handleReturnUnit', () async {
    await handleReturnUnit();
  });
  test('dart call handleReturnUnitSync', () {
    handleReturnUnit();
  });

  test('dart call handleString', () async {
    expect(await handleString(s: "Hello, world!"), "Hello, world!Hello, world!");
  });
  test('dart call handleString with nul-containing string', () async {
    expect(await handleString(s: "Hello\u0000world!"), isWeb ? "Hello\u0000world!Hello\u0000world!" : "");
  });

  test('dart call handleStringSync', () {
    expect(handleStringSync(s: "Hello, world!"), "Hello, world!Hello, world!");
  });
  test('dart call handleStringSync with nul-containing string', () {
    expect(handleStringSync(s: "Hello\u0000world!"), isWeb ? "Hello\u0000world!Hello\u0000world!" : "");
  });

  test('dart call handleVecU8', () async {
    final len = 100000;
    expect(
        await handleVecU8(v: Uint8List.fromList(List.filled(len, 127))), Uint8List.fromList(List.filled(len * 2, 127)));
  });
  test('dart call handleVecU8Sync', () {
    final len = 100000;
    expect(
        handleVecU8Sync(v: Uint8List.fromList(List.filled(len, 127))), Uint8List.fromList(List.filled(len * 2, 127)));
  });

  test('dart call handleVecOfPrimitive', () async {
    final n = 10000;
    final resp = await handleVecOfPrimitive(n: n);
    expect(resp.uint8List, Uint8List.fromList(List.filled(n, 42)));
    expect(resp.int8List, Int8List.fromList(List.filled(n, 42)));
    expect(resp.uint16List, Uint16List.fromList(List.filled(n, 42)));
    expect(resp.int16List, Int16List.fromList(List.filled(n, 42)));
    expect(resp.uint32List, Uint32List.fromList(List.filled(n, 42)));
    expect(resp.int32List, Int32List.fromList(List.filled(n, 42)));
    expect(resp.float32List, Float32List.fromList(List.filled(n, 42)));
    expect(resp.float64List, Float64List.fromList(List.filled(n, 42)));
    expect(resp.uint64List, Uint64List.fromList(List.filled(n, 42)));
    expect(resp.int64List, Int64List.fromList(List.filled(n, 42)));
    expect(resp.boolList, List.filled(n, true));
  });
  test('dart call handleVecOfPrimitiveSync', () {
    final n = 10000;
    final resp = handleVecOfPrimitiveSync(n: n);
    expect(resp.uint8List, Uint8List.fromList(List.filled(n, 42)));
    expect(resp.int8List, Int8List.fromList(List.filled(n, 42)));
    expect(resp.uint16List, Uint16List.fromList(List.filled(n, 42)));
    expect(resp.int16List, Int16List.fromList(List.filled(n, 42)));
    expect(resp.uint32List, Uint32List.fromList(List.filled(n, 42)));
    expect(resp.int32List, Int32List.fromList(List.filled(n, 42)));
    expect(resp.float32List, Float32List.fromList(List.filled(n, 42)));
    expect(resp.float64List, Float64List.fromList(List.filled(n, 42)));
    expect(resp.uint64List, Uint64List.fromList(List.filled(n, 42)));
    expect(resp.int64List, Int64List.fromList(List.filled(n, 42)));
    expect(resp.boolList, List.filled(n, true));
  });

  test('dart call handleZeroCopyVecOfPrimitive', () async {
    final n = 10000;
    final resp = await handleZeroCopyVecOfPrimitive(n: n);
    expect(resp.uint8List, Uint8List.fromList(List.filled(n, 42)));
    expect(resp.int8List, Int8List.fromList(List.filled(n, 42)));
    expect(resp.uint16List, Uint16List.fromList(List.filled(n, 42)));
    expect(resp.int16List, Int16List.fromList(List.filled(n, 42)));
    expect(resp.uint32List, Uint32List.fromList(List.filled(n, 42)));
    expect(resp.int32List, Int32List.fromList(List.filled(n, 42)));
    expect(resp.float32List, Float32List.fromList(List.filled(n, 42)));
    expect(resp.float64List, Float64List.fromList(List.filled(n, 42)));
    expect(resp.uint64List, Uint64List.fromList(List.filled(n, 42)));
    expect(resp.int64List, Int64List.fromList(List.filled(n, 42)));
  });
  test('dart call handleZeroCopyVecOfPrimitiveSync', () {
    final n = 10000;
    final resp = handleZeroCopyVecOfPrimitiveSync(n: n);
    expect(resp.uint8List, Uint8List.fromList(List.filled(n, 42)));
    expect(resp.int8List, Int8List.fromList(List.filled(n, 42)));
    expect(resp.uint16List, Uint16List.fromList(List.filled(n, 42)));
    expect(resp.int16List, Int16List.fromList(List.filled(n, 42)));
    expect(resp.uint32List, Uint32List.fromList(List.filled(n, 42)));
    expect(resp.int32List, Int32List.fromList(List.filled(n, 42)));
    expect(resp.float32List, Float32List.fromList(List.filled(n, 42)));
    expect(resp.float64List, Float64List.fromList(List.filled(n, 42)));
    expect(resp.uint64List, Uint64List.fromList(List.filled(n, 42)));
    expect(resp.int64List, Int64List.fromList(List.filled(n, 42)));
  });

  test('dart call handleStruct', () async {
    final structResp =
        await handleStruct(arg: MySize(width: 42, height: 100), boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
  });
  test('dart call handleStructSync', () {
    final structResp = handleStructSync(arg: MySize(width: 42, height: 100), boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
  });

  test('dart call handleStructSyncFreezed', () {
    final structResp = handleStructSyncFreezed(
        arg: MySizeFreezed(width: 42, height: 100), boxed: MySizeFreezed(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
    // Only freezed classes have copyWith
    expect(structResp.copyWith, isNotNull);
  });

  test('dart call handleNewtype', () async {
    final newtypeResp = await handleNewtype(arg: NewTypeInt(field0: 42));
    expect(newtypeResp.field0, 84);
  });
  test('dart call handleNewtypeSync', () {
    final newtypeResp = handleNewtypeSync(arg: NewTypeInt(field0: 42));
    expect(newtypeResp.field0, 84);
  });

  test('dart call handleListOfStruct', () async {
    final listOfStructResp =
        await handleListOfStruct(l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
    expect(listOfStructResp.length, 4);
    expect(listOfStructResp[0].width, 42);
    expect(listOfStructResp[1].width, 420);
    expect(listOfStructResp[2].width, 42);
    expect(listOfStructResp[3].width, 420);
  });
  test('dart call handleListOfStructSync', () {
    final listOfStructResp =
        handleListOfStructSync(l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
    expect(listOfStructResp.length, 4);
    expect(listOfStructResp[0].width, 42);
    expect(listOfStructResp[1].width, 420);
    expect(listOfStructResp[2].width, 42);
    expect(listOfStructResp[3].width, 420);
  });

  test('dart call handleStringList', () async {
    final names = await handleStringList(names: ['Steve', 'Bob', 'Alex']);
    expect(names, ['Steve', 'Bob', 'Alex']);
  });
  test('dart call handleStringListSync', () {
    final names = handleStringListSync(names: ['Steve', 'Bob', 'Alex']);
    expect(names, ['Steve', 'Bob', 'Alex']);
  });

  testComplexStruct(MyTreeNode complexStructResp, {required int arrLen}) {
    expect(complexStructResp.valueI32, 100);
    expect(complexStructResp.valueVecU8, List.filled(arrLen, 100));
    expect(complexStructResp.children[0].valueVecU8, List.filled(arrLen, 110));
    expect(complexStructResp.children[0].children[0].valueVecU8, List.filled(arrLen, 111));
    expect(complexStructResp.children[1].valueVecU8, List.filled(arrLen, 120));
  }

  test('dart call handleComplexStruct', () async {
    final arrLen = 5;
    final complexStructResp = await handleComplexStruct(s: _createMyTreeNode(arrLen: arrLen));
    testComplexStruct(complexStructResp, arrLen: arrLen);
  });

  test('dart call handleNestedStruct', () async {
    final r = await handleNestedStruct(s: _createMyNestedStruct());
    testComplexStruct(r.treeNode, arrLen: 5);
    expect(r.weekday, Weekdays.friday);
  });

  test('dart call handleComplexStructSync', () {
    final arrLen = 5;
    final complexStructResp = handleComplexStructSync(s: _createMyTreeNode(arrLen: arrLen));
    expect(complexStructResp.valueI32, 100);
    expect(complexStructResp.valueVecU8, List.filled(arrLen, 100));
    expect(complexStructResp.children[0].valueVecU8, List.filled(arrLen, 110));
    expect(complexStructResp.children[0].children[0].valueVecU8, List.filled(arrLen, 111));
    expect(complexStructResp.children[1].valueVecU8, List.filled(arrLen, 120));
  });

  // Test if sync return is working as expected.
  test('dart call handle_sync_return', () async {
    expect(handleSyncReturn(mode: 'NORMAL'), List.filled(100, 42));

    try {
      handleSyncReturn(mode: 'RESULT_ERR');
      fail("exception not thrown");
    } on FrbAnyhowException catch (e) {
      print('dart catch anyhow e: $e');
    }

    try {
      handleSyncReturn(mode: 'PANIC');
      fail("exception not thrown");
    } on PanicException catch (e) {
      print('dart catch panic e: $e');
    }
  });

  test('dart call handle_stream', () async {
    final stream = handleStream(arg: 'hello');
    var cnt = 0;
    await for (final value in stream) {
      print("output from handle_stream's stream: $value");
      cnt++;
    }
    expect(cnt, 10);
  });

  Future<void> testHandleStream(
      Stream<Log> Function({dynamic hint, required int key, required int max}) handleStreamFunction) async {
    final max = 5;
    final key = 8;
    final stream = handleStreamFunction(key: key, max: max);
    var cnt = 0;
    await for (final value in stream) {
      print("output from handle_stream_x's stream: $value");
      expect(value.key, key);
      cnt++;
    }
    expect(cnt, max);
  }

  test('dart call handle_stream_sink_at_1', () {
    testHandleStream(handleStreamSinkAt1);
  });

  test('dart call handle_stream_sink_at_2', () {
    testHandleStream(handleStreamSinkAt2);
  });

  test('dart call handle_stream_sink_at_3', () {
    testHandleStream(handleStreamSinkAt3);
  });

  void testAppSettings(ApplicationSettings settings) {
    expect(settings.version, "1.0.0-rc.1");
    expect(settings.mode, ApplicationMode.standalone);
    expect(settings.env.vars[0].field0, "myenv");
  }

  test('dart call returnErr', () async {
    try {
      await returnErr();
      fail("exception not thrown");
    } on FrbAnyhowException catch (e) {
      print('dart catch e: $e');
    }
  });

  test('dart call returnPanic', () async {
    try {
      await returnPanic();
      fail("exception not thrown");
    } catch (e) {
      print('dart catch e: $e');
      expect(e, isA<PanicException>());
    }
  });

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
      expect(ret.int64, loopFor, reason: 'int64');
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

  test('dart call handleVecOfOpts', () async {
    const loops = 20;
    var opt = OptVecs(i32: [], enums: [Weekdays.monday], strings: ['foo'], buffers: []);
    for (var i = 0; i < loops; i++) {
      opt = await handleVecOfOpts(opt: opt);
    }
    final nulls = List.filled(loops, null);
    expect(opt.i32, nulls);
    expect(opt.enums, [Weekdays.monday, for (final val in nulls) val]);
    expect(opt.strings, ['foo', for (final val in nulls) val]);
    expect(opt.buffers, nulls);
  });

  test('dart call handleReturnEnum', () async {
    expect(await handleReturnEnum(input: "Tuesday"), Weekdays.tuesday);
    expect(await handleReturnEnum(input: "Foreverday"), null);
  });

  test('dart call handleEnumParameter', () async {
    expect(await handleEnumParameter(weekday: Weekdays.saturday), Weekdays.saturday);
  });

  test('dart call handleEnumParameter', () async {
    expect(handleEnumSyncFreezed(value: MyEnumFreezed.a(1)), MyEnumFreezed.b('hello'));
  });

  test('dart call handleEnumStruct', () async {
    expect(await handleEnumStruct(val: KitchenSink_Empty()), KitchenSink_Empty());
    expect(
      await handleEnumStruct(
        val: KitchenSink_Primitives(int32: 0, float64: 1, boolean: false),
      ),
      KitchenSink_Primitives(int32: 1, float64: 2, boolean: true),
    );
    expect(
      await handleEnumStruct(val: KitchenSink_Optional(null, 0)),
      KitchenSink_Optional(null, 1),
    );
    expect(
      await handleEnumStruct(val: KitchenSink_Buffer(Uint8List.fromList([]))),
      KitchenSink_Buffer(Uint8List.fromList([1])),
    );
    expect(
      await handleEnumStruct(val: KitchenSink_Enums(Weekdays.monday)),
      KitchenSink_Enums(Weekdays.tuesday),
    );
    expect(
      await handleEnumStruct(val: const KitchenSink.nested(0, KitchenSink.empty())),
      const KitchenSink.nested(1, KitchenSink.empty()),
    );
  });

  test('dart call useImportedStruct()', () async {
    expect(
      await useImportedStruct(myStruct: MyStruct(content: false)),
      false,
    );
    expect(
      await useImportedStruct(myStruct: MyStruct(content: true)),
      true,
    );
  });

  test('dart call useImportedEnum()', () async {
    expect(
      await useImportedEnum(myEnum: MyEnum.False),
      false,
    );
    expect(
      await useImportedEnum(myEnum: MyEnum.True),
      true,
    );
  });

  test('loop and call many times', () async {
    var obj = _createMyTreeNode(arrLen: 5);
    for (var i = 0; i < 500; ++i) {
      obj = await handleComplexStruct(s: obj);
    }
  });

  test('dart call getUsize', () async {
    expect(await getUsize(u: 2), 2);
  });

  test('ConcatenateWith test', () async {
    final ConcatenateWith concatenateWith = ConcatenateWith(a: "hello ");
    final String concatenated = await concatenateWith.concatenate(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated = await ConcatenateWith.concatenateStatic(a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor = await ConcatenateWith.newConcatenateWith(a: "hello ");
    final String concatenated2 = await concatenatedConstructor.concatenate(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWith sumWith = SumWith(x: 3);
    final int sum = await sumWith.sum(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWith sumWith = await getSumStruct();
    final int sum = await sumWith.sum(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWith> sumWithList = await getSumArray(a: 12, b: 23, c: 67);
    expect(await sumWithList[0].sum(y: 23, z: 67), 12 + 23 + 67);
    expect(await sumWithList[1].sum(y: 12, z: 67), 12 + 23 + 67);
    expect(await sumWithList[2].sum(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWith concatenateWith = ConcatenateWith(a: "hello ");
    final int key = 10;
    final int max = 5;
    final stream = concatenateWith.handleSomeStreamSink(key: key, max: max);
    int cnt = 0;
    await for (final value in stream) {
      print("output from ConcatenateWith's stream: $value");
      expect(value.value, "hello $cnt");
      cnt++;
    }
    expect(cnt, max);
  });

  test('ConcatenateWith static stream sink test', () async {
    final int key = 10;
    final int max = 5;
    final stream = ConcatenateWith.handleSomeStaticStreamSink(key: key, max: max);
    int cnt = 0;
    await for (final value in stream) {
      print("output from ConcatenateWith's static stream: $value");
      expect(value.value, "$cnt");
      cnt++;
    }
    expect(cnt, max);
  });

  test('ConcatenateWith static stream sink at 1 test', () async {
    final stream = ConcatenateWith.handleSomeStaticStreamSinkSingleArg(bridge: api);
    expect(stream.toList(), completion([0, 1, 2, 3, 4]));
  });

  test('dart call multiplyByTen()', () async {
    expect(
      await multiplyByTen(measure: Measure.speed(Speed_GPS(10.0))),
      Measure.speed(Speed_GPS(100.0)),
    );
    expect(
      await multiplyByTen(measure: Measure.speed(Speed_Unknown())),
      null,
    );
    final skipMinified = releaseMode ? skipWeb('Minified names cannot be compared.') : null;
    expect((Speed_Unknown).toString(), 'Speed_Unknown', skip: skipMinified);
    expect((Speed_GPS).toString(), 'Speed_GPS', skip: skipMinified);
    expect((Distance_Unknown).toString(), 'Distance_Unknown', skip: skipMinified);
    expect((Distance_Map).toString(), 'Distance_Map', skip: skipMinified);
  });

  test('resolve module for old module system', () async {
    final o = await callOldModuleSystem();
    expect(o.field, 2);
  });
  test('resolve module for new module system', () async {
    final n = await callNewModuleSystem();
    expect(n.field, 1);
  });

  test('test empty struct', () async {
    final empty = Empty();
    final output = await emptyStruct(empty: empty);
    expect(output, isA<Empty>());
  });

  test('test abc', () async {
    final output1 = await testAbcEnum(abc: Abc.a(A(a: "test")));
    expect((output1 as Abc_A).field0.a, "test");

    final output2 = await testAbcEnum(abc: Abc.b(B(b: 1)));
    expect((output2 as Abc_B).field0.b, 1);

    final output3 = await testAbcEnum(abc: Abc.c(C(c: false)));
    expect((output3 as Abc_C).field0.c, false);

    final output4 = await testAbcEnum(abc: Abc.justInt(1));
    expect((output4 as Abc_JustInt).field0, 1);
  });

  group('Platform-specific support', () {
    test('Int64List', () {
      final list = Int64List.fromList([-1, -2, -3, -4, -5]);
      expect(list[0], BigInt.from(-1));
      expect(list.map((el) => el * el), MatchBigInt([1, 4, 9, 16, 25]));
      list[1] = -123;
      expect(list[1], BigInt.from(-123));
    });
    test('Uint64List', () {
      final list = Uint64List.fromList([1, 2, 3, 4, 5]);
      expect(list[0], BigInt.one);
      expect(list.map((el) => el * el), MatchBigInt([1, 4, 9, 16, 25]));
      list[1] = 123;
      expect(list[1], BigInt.from(123));
      list[1] += BigInt.one;
      expect(list[1], BigInt.from(124));
    });
    test('Lossless big buffers', () async {
      final list = await handleBigBuffers();
      expect(list.int64[0], BigInt.parse('-9223372036854775808'));
      expect(list.int64[1], BigInt.parse('9223372036854775807'));
      expect(list.uint64[0], BigInt.parse('0xFFFFFFFFFFFFFFFF'), reason: 'uint64');
    });
  });

  group('extended sync', () {
    test('check generator', () {
      expect(frbSyncGeneratorTest().runtimeType == FrbOpaqueSyncReturn, isTrue);
    });

    test('create', () {
      var data = syncCreateOpaque();
      data.dispose();
    });

    test('double call', () {
      var data = syncCreateSyncOpaque();
      expect(
          syncRunOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(
          syncRunOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.dispose();
    });

    test('call after drop', () {
      var data = syncCreateSyncOpaque();
      expect(
          syncRunOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.dispose();
      expect(() => syncRunOpaque(opaque: data), throwsA(isA<PanicException>()));
    });

    test('option', () async {
      var data = syncOption();
      var data2 = syncOptionNull();
      var data3 = syncOptionRustOpaque();
      var data4 = syncOptionDartOpaque(opaque: () => () => 'magic');
      expect(data, isNotNull);
      expect(data2, isNull);
      expect(data3, isNotNull);
      expect(data4, isNotNull);
      data3!.dispose();
    });

    test('nonclone', () async {
      var data = syncCreateNonClone();
      var data2 = await runNonClone(clone: data);
      expect(data2, "content");
      data.dispose();
    });

    test('void', () async {
      syncVoid();
    });

    test('unwrapped dart opaque', () async {
      String f() => "magic";
      var res = returnNonDroppableDartOpaque(opaque: f);
      expect(identical(res, f), isTrue);
    });
  });

  test('dart call return_dart_dynamic', () async {
    final data = await returnDartDynamic();
    expect(data, ['foo']);
  });

  test("dart call list_of_primitive_enums", () async {
    List<Weekdays> days = await listOfPrimitiveEnums(weekdays: Weekdays.values);
    expect(days, Weekdays.values);
  });

  test("dart call struct_with_enum_member", () async {
    final result = await testStructWithEnum(se: StructWithEnum(abc1: Abc.a(A(a: "aaa")), abc2: Abc.b(B(b: 999))));
    expect(result.abc1.whenOrNull(b: (B b) => b.b), 999);
    expect(result.abc2.whenOrNull(a: (A a) => a.a), "aaa");
  });

  test("dart call tuples", () async {
    expect(testTuple(), completion(('John', 0)));
    expect(testTuple(value: ('Bob', 42)), completion(('Hello Bob', 43)));
  });

  test("sync return mirror", () {
    final settings = syncReturnMirror();
    testAppSettings(settings);
  });

  test("macro struct", () async {
    var macroStruct = await macroStruct();
    expect(macroStruct.data, 123);
    macroStruct.nonFinalData = 321;
    expect(macroStruct.nonFinalData, 321);
  });

  group('Custom error (Result<T,E>)', () {
    // The first time a backtrace is created, symbol resolution
    // takes a significant amount of time.
    test('Throw CustomError', timeout: Timeout.factor(5), () {
      expect(returnErrCustomError(), throwsA(isA<CustomError>()));
    });

    test('Throw CustomStructError', () async {
      await expectLater(() async => await returnCustomStructError(), throwsA(isA<CustomStructError>()));
    });

    test('Throw sync CustomStructError', () {
      try {
        syncReturnCustomStructError();
      } on CustomStructError catch (e) {
        expect(e.message, "error message");
      }
    });

    test('Do not throw CustomStructError', () async {
      expect(await returnCustomStructOk(), 3);
    });

    test('Throw CustomStructError non static method', () async {
      await expectLater(() async => await CustomStruct(message: "hello").nonstaticReturnCustomStructError(),
          throwsA(isA<CustomStructError>()));
    });

    test('Do not throw CustomStructError non static method', () async {
      expect(await CustomStruct(message: "hello").nonstaticReturnCustomStructOk(), 3);
    });

    test('Throw CustomStructError static method', () async {
      await expectLater(
          () async => await CustomStruct.staticReturnCustomStructError(bridge: api), throwsA(isA<CustomStructError>()));
    });

    test('Do not throw CustomStructError static method', () async {
      expect(await CustomStruct.staticReturnCustomStructOk(bridge: api), 3);
    });

    test('Throw CustomNestedError1', () async {
      await expectLater(() async => await returnCustomNestedError1(),
          throwsA(CustomNestedError1.errorNested(CustomNestedError2.customNested2Number(3))));
    });

    test('Throw CustomNestedError1 variant 1', () async {
      await expectLater(
          () async => await returnCustomNestedError1Variant1(), throwsA(CustomNestedError1.customNested1("custom")));
    });

    test('Throw CustomNestedError2', () async {
      await expectLater(
          () async => await returnCustomNestedError2(), throwsA(CustomNestedError2.customNested2("custom")));
    });

    test('Throw CustomError variant 0', () async {
      await expectLater(() async => await returnErrorVariant(variant: 0), throwsA(isA<CustomError>()));
    });

    test('Throw CustomError variant 1', () async {
      await expectLater(() async => await returnErrorVariant(variant: 1), throwsA(isA<CustomError>()));
    });

    test('Do not throw CustomError', () async {
      expect(await returnOkCustomError(), 3);
    });

    test('Throw CustomError static method', () async {
      await expectLater(
          () async => await SomeStruct.staticReturnErrCustomError(bridge: api), throwsA(isA<CustomError>()));
    });

    test('Throw CustomError static method, verifies implements Frb', () async {
      await expectLater(
          () async => await SomeStruct.staticReturnErrCustomError(bridge: api), throwsA(isA<FrbException>()));
    });

    test('Do not throw CustomError static method', () async {
      expect(await SomeStruct.staticReturnOkCustomError(bridge: api), 3);
    });

    test('Do not throw CustomError', () async {
      expect(await returnOkCustomError(), 3);
    });

    test('Throw CustomError non-static method', () async {
      await expectLater(
          () async => await SomeStruct(value: 7).nonStaticReturnErrCustomError(), throwsA(isA<CustomError>()));
      bool didCatch = false;
      try {
        await SomeStruct(value: 7).nonStaticReturnErrCustomError();
      } catch (e) {
        final FrbBacktracedException ex = e as FrbBacktracedException;
        print("backtrace: ${ex.backtrace}");
        assert(ex.backtrace.contains("wire_non_static_return_err_custom_error__method__SomeStruct::"));
        didCatch = true;
      }
      assert(didCatch);
    });

    test('Do not throw CustomError non-static method', () async {
      expect(await SomeStruct(value: 6).nonStaticReturnOkCustomError(), 6);
    });

    test('Throw anyhow error', () async {
      await expectLater(() async => await throwAnyhow(), throwsA(isA<FrbException>()));
      try {
        await throwAnyhow();
      } catch (e) {
        final FrbAnyhowException p = e as FrbAnyhowException;
        print("anyhow error: ${p.anyhow}");
        assert(p.anyhow.contains("anyhow error"));
      }
    });

    test('Function with custom result panics', () async {
      await expectLater(() async => await panicWithCustomResult(), throwsA(isA<FrbException>()));
      try {
        await panicWithCustomResult();
      } catch (e) {
        final PanicException p = e as PanicException;
        print("panic error: ${p.error}");
        assert(p.error.contains("just a panic"));
      }
    });

    test('Stream sink throw anyhow error', () async {
      expect(
        () async {
          await for (final _ in streamSinkThrowAnyhow()) {}
        },
        throwsA(isA<FrbAnyhowException>().having((e) => e.toString(), 'toString', 'FrbAnyhowException(anyhow error)')),
      );
    });
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

MyTreeNode _createMyTreeNode({required int arrLen}) {
  return MyTreeNode(
    valueI32: 100,
    valueVecU8: Uint8List.fromList(List.filled(arrLen, 100)),
    valueBoolean: true,
    children: [
      MyTreeNode(
        valueI32: 110,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 110)),
        valueBoolean: true,
        children: [
          MyTreeNode(
            valueI32: 111,
            valueVecU8: Uint8List.fromList(List.filled(arrLen, 111)),
            valueBoolean: true,
            children: [],
          ),
        ],
      ),
      MyTreeNode(
        valueI32: 120,
        valueVecU8: Uint8List.fromList(List.filled(arrLen, 120)),
        valueBoolean: true,
        children: [],
      ),
    ],
  );
}

MyNestedStruct _createMyNestedStruct() {
  return MyNestedStruct(treeNode: _createMyTreeNode(arrLen: 5), weekday: Weekdays.friday);
}

class MatchBigInt extends CustomMatcher {
  MatchBigInt(matcher) : super("is a numeric", "value", _featureValueOf(matcher));

  @override
  Object? featureValueOf(actual) => _featureValueOf(actual);

  static Object? _featureValueOf(actual) {
    if (actual is Iterable) return actual.map(_featureValueOf);
    if (actual is int) return BigInt.from(actual);
    return actual;
  }
}

Uint8List createLargeList({required int mb}) => Uint8List(1000000 * mb);

// vim:expandtab:ts=2:sw=2
