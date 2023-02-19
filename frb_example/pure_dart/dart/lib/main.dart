import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';
import 'package:uuid/uuid.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'bridge_definitions.dart';

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
  tearDownAll(() => api.dispose());

  test('dart call simpleAdder', () async {
    expect(await api.simpleAdder(a: 42, b: 100), 142);
  });

  test('dart call simpleAdderSync', () {
    expect(api.simpleAdderSync(a: 42, b: 100), 142);
  });

  test('dart call primitiveTypes', () async {
    expect(
        await api.primitiveTypes(myI32: 123, myI64: 10000000000000, myF64: 12345678901234567890.123, myBool: true), 42);
  });

  test('dart call optional primitiveTypes', () async {
    expect(await api.primitiveOptionalTypes(myI32: null, myI64: null, myF64: null, myBool: null), 0);
    expect(await api.primitiveOptionalTypes(myI32: 0, myI64: 0, myF64: 0, myBool: false), 4);
    expect(await api.primitiveOptionalTypes(myI32: 123, myI64: 123, myF64: 123, myBool: true), 4);
  });

  test('dart call primitiveTypesSync', () {
    expect(
        api.primitiveTypesSync(myI32: 123, myI64: 10000000000000, myF64: 12345678901234567890.123, myBool: true), 42);
  });

  test('dart call primitiveU32', () async {
    expect(await api.primitiveU32(myU32: 0xff112233), 0xfe112233);
  });
  test('dart call primitiveU32Sync', () {
    expect(api.primitiveU32Sync(myU32: 0xff112233), 0xfe112233);
  });

  test('dart call handleReturnUnit', () async {
    await api.handleReturnUnit();
  });
  test('dart call handleReturnUnitSync', () {
    api.handleReturnUnit();
  });

  test('dart call handleString', () async {
    expect(await api.handleString(s: "Hello, world!"), "Hello, world!Hello, world!");
  });
  test('dart call handleString with nul-containing string', () async {
    expect(await api.handleString(s: "Hello\u0000world!"), isWeb ? "Hello\u0000world!Hello\u0000world!" : "");
  });

  test('dart call handleStringSync', () {
    expect(api.handleStringSync(s: "Hello, world!"), "Hello, world!Hello, world!");
  });
  test('dart call handleStringSync with nul-containing string', () {
    expect(api.handleStringSync(s: "Hello\u0000world!"), isWeb ? "Hello\u0000world!Hello\u0000world!" : "");
  });

  test('dart call handleVecU8', () async {
    final len = 100000;
    expect(await api.handleVecU8(v: Uint8List.fromList(List.filled(len, 127))),
        Uint8List.fromList(List.filled(len * 2, 127)));
  });
  test('dart call handleVecU8Sync', () {
    final len = 100000;
    expect(api.handleVecU8Sync(v: Uint8List.fromList(List.filled(len, 127))),
        Uint8List.fromList(List.filled(len * 2, 127)));
  });

  test('dart call handleVecOfPrimitive', () async {
    final n = 10000;
    final resp = await api.handleVecOfPrimitive(n: n);
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
  test('dart call handleVecOfPrimitiveSync', () {
    final n = 10000;
    final resp = api.handleVecOfPrimitiveSync(n: n);
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

  test('dart call handleZeroCopyVecOfPrimitive', () async {
    final n = 10000;
    final resp = await api.handleZeroCopyVecOfPrimitive(n: n);
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
    final resp = api.handleZeroCopyVecOfPrimitiveSync(n: n);
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
        await api.handleStruct(arg: MySize(width: 42, height: 100), boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
  });
  test('dart call handleStructSync', () {
    final structResp =
        api.handleStructSync(arg: MySize(width: 42, height: 100), boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
  });

  test('dart call handleNewtype', () async {
    final newtypeResp = await api.handleNewtype(arg: NewTypeInt(field0: 42));
    expect(newtypeResp.field0, 84);
  });
  test('dart call handleNewtypeSync', () {
    final newtypeResp = api.handleNewtypeSync(arg: NewTypeInt(field0: 42));
    expect(newtypeResp.field0, 84);
  });

  test('dart call handleListOfStruct', () async {
    final listOfStructResp =
        await api.handleListOfStruct(l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
    expect(listOfStructResp.length, 4);
    expect(listOfStructResp[0].width, 42);
    expect(listOfStructResp[1].width, 420);
    expect(listOfStructResp[2].width, 42);
    expect(listOfStructResp[3].width, 420);
  });
  test('dart call handleListOfStructSync', () {
    final listOfStructResp =
        api.handleListOfStructSync(l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
    expect(listOfStructResp.length, 4);
    expect(listOfStructResp[0].width, 42);
    expect(listOfStructResp[1].width, 420);
    expect(listOfStructResp[2].width, 42);
    expect(listOfStructResp[3].width, 420);
  });

  test('dart call handleStringList', () async {
    final names = await api.handleStringList(names: ['Steve', 'Bob', 'Alex']);
    expect(names, ['Steve', 'Bob', 'Alex']);
  });
  test('dart call handleStringListSync', () {
    final names = api.handleStringListSync(names: ['Steve', 'Bob', 'Alex']);
    expect(names, ['Steve', 'Bob', 'Alex']);
  });

  test('dart call handleComplexStruct', () async {
    final arrLen = 5;
    final complexStructResp = await api.handleComplexStruct(s: _createMyTreeNode(arrLen: arrLen));
    expect(complexStructResp.valueI32, 100);
    expect(complexStructResp.valueVecU8, List.filled(arrLen, 100));
    expect(complexStructResp.children[0].valueVecU8, List.filled(arrLen, 110));
    expect(complexStructResp.children[0].children[0].valueVecU8, List.filled(arrLen, 111));
    expect(complexStructResp.children[1].valueVecU8, List.filled(arrLen, 120));
  });
  test('dart call handleComplexStructSync', () {
    final arrLen = 5;
    final complexStructResp = api.handleComplexStructSync(s: _createMyTreeNode(arrLen: arrLen));
    expect(complexStructResp.valueI32, 100);
    expect(complexStructResp.valueVecU8, List.filled(arrLen, 100));
    expect(complexStructResp.children[0].valueVecU8, List.filled(arrLen, 110));
    expect(complexStructResp.children[0].children[0].valueVecU8, List.filled(arrLen, 111));
    expect(complexStructResp.children[1].valueVecU8, List.filled(arrLen, 120));
  });

  // Test if sync return is working as expected.
  test('dart call handle_sync_return', () async {
    expect(api.handleSyncReturn(mode: 'NORMAL'), List.filled(100, 42));

    for (final mode in ['RESULT_ERR', 'PANIC']) {
      try {
        api.handleSyncReturn(mode: mode);
        fail("exception not thrown");
      } on FfiException catch (e) {
        print('dart catch e: $e');
      }
    }
  });

  test('dart call handle_stream', () async {
    final stream = api.handleStream(arg: 'hello');
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
    testHandleStream(api.handleStreamSinkAt1);
  });

  test('dart call handle_stream_sink_at_2', () {
    testHandleStream(api.handleStreamSinkAt2);
  });

  test('dart call handle_stream_sink_at_3', () {
    testHandleStream(api.handleStreamSinkAt3);
  });

  test('dart call returnErr', () async {
    try {
      await api.returnErr();
      fail("exception not thrown");
    } on FfiException catch (e) {
      print('dart catch e: $e');
    }
  });

  test('dart call returnPanic', () async {
    try {
      await api.returnPanic();
      fail("exception not thrown");
    } catch (e) {
      print('dart catch e: $e');
      expect(e, isA<FfiException>());
    }
  });

  test('dart call handleOptionalReturn', () async {
    expect((await api.handleOptionalReturn(left: 1, right: 1))!, 1);
    expect(await api.handleOptionalReturn(left: 2, right: 0), null);
  });

  test('dart call handleOptionalStruct', () async {
    {
      expect(await api.handleOptionalStruct(), null);
    }

    {
      final message = 'Hello there.';
      final ret = await api.handleOptionalStruct(document: message);
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
    expect(await api.handleOptionalIncrement(), null);
    {
      var ret = await api.handleOptionalIncrement(opt: ExoticOptionals(attributesNullable: []));
      if (ret == null) fail('increment returned null for non-null params');
      final loopFor = 20;
      for (var i = 1; i < loopFor; i++) {
        ret = await api.handleOptionalIncrement(opt: ret);
      }
      if (ret == null) fail('ret nulled after loop');
      expect(ret.int32, loopFor, reason: 'int32');
      expect(ret.int64, loopFor, reason: 'int64');
      expect(ret.float64, loopFor, reason: 'float64');
      expect(ret.boolean, false);
      expect(ret.zerocopy?.length, loopFor);
      expect(ret.int8List?.length, loopFor);
      expect(ret.uint8List?.length, loopFor);
      expect(ret.attributesNullable.length, loopFor);
      expect(ret.nullableAttributes?.length, loopFor);
      expect(ret.newtypeint?.field0, loopFor, reason: 'NewTypeInt');
    }
  });

  test('dart call handleIncrementBoxedOptional', () async {
    {
      expect(await api.handleIncrementBoxedOptional(), 42);
    }

    {
      var ret = 0.0;
      final loopFor = 100;
      for (var i = 0; i < loopFor; i++) {
        ret = await api.handleIncrementBoxedOptional(opt: ret);
      }
      expect(ret, loopFor);
    }
  });

  test('dart call handleOptionBoxArguments', () async {
    print(await api.handleOptionBoxArguments());

    {
      final optional10 = await api.handleOptionBoxArguments(
        boolbox: true,
        structbox: await api.handleOptionalIncrement(opt: ExoticOptionals(attributesNullable: [])),
      );
      print(optional10);
    }
  });

  test('dart call handleReturnEnum', () async {
    expect(await api.handleReturnEnum(input: "Tuesday"), Weekdays.Tuesday);
    expect(await api.handleReturnEnum(input: "Foreverday"), null);
  });

  test('dart call handleEnumParameter', () async {
    expect(await api.handleEnumParameter(weekday: Weekdays.Saturday), Weekdays.Saturday);
  });

  test('dart call handleEnumStruct', () async {
    expect(await api.handleEnumStruct(val: KitchenSink_Empty()), KitchenSink_Empty());
    expect(
      await api.handleEnumStruct(
        val: KitchenSink_Primitives(int32: 0, float64: 1, boolean: false),
      ),
      KitchenSink_Primitives(int32: 1, float64: 2, boolean: true),
    );
    expect(
      await api.handleEnumStruct(val: KitchenSink_Optional(null, 0)),
      KitchenSink_Optional(null, 1),
    );
    expect(
      await api.handleEnumStruct(val: KitchenSink_Buffer(Uint8List.fromList([]))),
      KitchenSink_Buffer(Uint8List.fromList([1])),
    );
    expect(
      await api.handleEnumStruct(val: KitchenSink_Enums(Weekdays.Monday)),
      KitchenSink_Enums(Weekdays.Tuesday),
    );
    expect(
      await api.handleEnumStruct(val: KitchenSink_Nested(KitchenSink_Empty(), 0)),
      KitchenSink_Nested(KitchenSink_Empty(), 1),
    );
  });

  test('dart call useImportedStruct()', () async {
    expect(
      await api.useImportedStruct(myStruct: MyStruct(content: false)),
      false,
    );
    expect(
      await api.useImportedStruct(myStruct: MyStruct(content: true)),
      true,
    );
  });

  test('dart call useImportedEnum()', () async {
    expect(
      await api.useImportedEnum(myEnum: MyEnum.False),
      false,
    );
    expect(
      await api.useImportedEnum(myEnum: MyEnum.True),
      true,
    );
  });

  test('dart call getAppSettings()', () async {
    var settings = await api.getAppSettings();
    expect(settings.version, "1.0.0-rc.1");
    expect(settings.mode, ApplicationMode.Standalone);
    expect(settings.env.vars[0].field0, "myenv");
  });

  test('dart call isAppEmbedded()', () async {
    expect(
        await api.isAppEmbedded(
            appSettings: ApplicationSettings(
                name: "from dart",
                version: "XX",
                mode: ApplicationMode.Embedded,
                env: ApplicationEnv(vars: [ApplicationEnvVar(field0: "sendback", field1: true)]))),
        true);
  });

  test('dart call getMessage()', () async {
    var message = await api.getMessage();
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
    var numbers = await api.repeatNumber(num: 1, times: 10);
    expect(numbers.field0.toList(), Int32List.fromList([1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
  });

  test('dart call repeatSequence()', () async {
    var sequences = await api.repeatSequence(seq: 1, times: 10);
    expect(sequences.field0.toList(), Int32List.fromList([1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
  });

  test('dart call firstNumber()', () async {
    var numbers = Numbers(field0: Int32List.fromList([1]));
    var first = await api.firstNumber(nums: numbers);
    expect(first, 1);
  });

  test('dart call firstSequence()', () async {
    var sequences = Sequences(field0: Int32List.fromList([1]));
    var first = await api.firstSequence(seqs: sequences);
    expect(first, 1);
  });

  test('loop and call many times', () async {
    var obj = _createMyTreeNode(arrLen: 5);
    for (var i = 0; i < 500; ++i) {
      obj = await api.handleComplexStruct(s: obj);
    }
  });

  test('dart call getArray()', () async {
    var array = await api.getArray();
    expect(array, [1, 2, 3, 4, 5]);
  });

  test('dart call getComplexArray()', () async {
    final points = await api.getComplexArray();

    expect(points[0].x, 1.0);
    expect(points[1].x, 2.0);
  });

  test('dart call getUsize', () async {
    expect(await api.getUsize(u: 2), 2);
  });

  test('dart check that non-final field is modifiable', () {
    var customized = Customized(finalField: "finalField", nonFinalField: "nonFinalField");
    expect(customized.nonFinalField, "nonFinalField");
    customized.nonFinalField = "changed";
    expect(customized.nonFinalField, "changed");
  });

  test('dart call next_user_id to test metadata annotations', () async {
    UserId userId = UserId(value: 11);
    expect(await api.nextUserId(userId: userId), UserId(value: 12));
  });

  test('dart register event listener & create event with delay', () async {
    expectLater(api.registerEventListener(), emits(Event(address: 'foo', payload: 'bar')));
    await Future.delayed(const Duration(milliseconds: 20));
    await api.createEvent(address: 'foo', payload: 'bar');
    await api.closeEventListener();
  });

  test('ConcatenateWith test', () async {
    final ConcatenateWith concatenateWith = ConcatenateWith(a: "hello ", bridge: api);
    final String concatenated = await concatenateWith.concatenate(b: "world");
    expect(concatenated, equals("hello world"));

    final staticConcatenated = await ConcatenateWith.concatenateStatic(bridge: api, a: "hello ", b: "world");
    expect(staticConcatenated, equals("hello world"));

    final concatenatedConstructor = await ConcatenateWith.newConcatenateWith(bridge: api, a: "hello ");
    final String concatenated2 = await concatenatedConstructor.concatenate(b: "world");
    expect(concatenated2, equals("hello world"));
  });

  test('SumWith test', () async {
    final SumWith sumWith = SumWith(bridge: api, x: 3);
    final int sum = await sumWith.sum(y: 1, z: 5);
    expect(sum, equals(3 + 1 + 5));
  });

  test('return SumWith test', () async {
    final SumWith sumWith = await api.getSumStruct();
    final int sum = await sumWith.sum(y: 1, z: 5);
    expect(sum, equals(21 + 1 + 5));
  });

  test('return SumWith array test', () async {
    final List<SumWith> sumWithList = await api.getSumArray(a: 12, b: 23, c: 67);
    expect(await sumWithList[0].sum(y: 23, z: 67), 12 + 23 + 67);
    expect(await sumWithList[1].sum(y: 12, z: 67), 12 + 23 + 67);
    expect(await sumWithList[2].sum(y: 12, z: 23), 12 + 23 + 67);
  });

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWith concatenateWith = ConcatenateWith(a: "hello ", bridge: api);
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
    final stream = ConcatenateWith.handleSomeStaticStreamSink(bridge: api, key: key, max: max);
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
      await api.multiplyByTen(measure: Measure.speed(Speed_GPS(10.0))),
      Measure.speed(Speed_GPS(100.0)),
    );
    expect(
      await api.multiplyByTen(measure: Measure.speed(Speed_Unknown())),
      null,
    );
    final skipMinified = releaseMode ? skipWeb('Minified names cannot be compared.') : null;
    expect((Speed_Unknown).toString(), 'Speed_Unknown', skip: skipMinified);
    expect((Speed_GPS).toString(), 'Speed_GPS', skip: skipMinified);
    expect((Distance_Unknown).toString(), 'Distance_Unknown', skip: skipMinified);
    expect((Distance_Map).toString(), 'Distance_Map', skip: skipMinified);
  });

  test('resolve module for old module system', () async {
    final o = await api.callOldModuleSystem();
    expect(o.field, 2);
  });
  test('resolve module for new module system', () async {
    final n = await api.callNewModuleSystem();
    expect(n.field, 1);
  });

  test('test empty struct', () async {
    final empty = await api.emptyStruct();
    expect(empty, Empty());
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
      final list = await api.handleBigBuffers();
      expect(list.int64[0], BigInt.parse('-9223372036854775808'));
      expect(list.int64[1], BigInt.parse('9223372036854775807'));
      expect(list.uint64[0], BigInt.parse('0xFFFFFFFFFFFFFFFF'), reason: 'uint64');
    });
  });

  group('chrono feature', () {
    test('DateTime<Utc>', () async {
      final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
      final resp = await api.datetimeUtc(d: date);
      expect(resp.year, date.year);
      expect(resp.month, date.month);
      expect(resp.day, date.day);
      expect(resp.hour, date.hour);
      expect(resp.minute, date.minute);
      expect(resp.second, date.second);
      expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
      expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
    });
    test('DateTime<Local>', () async {
      final date = DateTime(2022, 09, 10, 20, 48, 53, 123, 456);
      final resp = await api.datetimeLocal(d: date);
      expect(resp.year, date.year);
      expect(resp.month, date.month);
      expect(resp.day, date.day);
      expect(resp.hour, date.hour);
      expect(resp.minute, date.minute);
      expect(resp.second, date.second);
      expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
      expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
    });
    test('NaiveDateTime', () async {
      final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
      final resp = await api.naivedatetime(d: date);
      expect(resp.year, date.year);
      expect(resp.month, date.month);
      expect(resp.day, date.day);
      expect(resp.hour, date.hour);
      expect(resp.minute, date.minute);
      expect(resp.second, date.second);
      expect(resp.millisecondsSinceEpoch, date.millisecondsSinceEpoch);
      expect(resp.microsecondsSinceEpoch, date.microsecondsSinceEpoch);
    });
    test('Empty DateTime', () async {
      final resp = await api.optionalEmptyDatetimeUtc(d: null);
      expect(resp, null);
    });
    test('Duration', () async {
      final duration = Duration(hours: 4);
      final resp = await api.duration(d: duration);
      expect(resp.inHours, duration.inHours);
    });
    test('nested chrono types', () async {
      const duration = Duration(hours: 4);
      final naive = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
      final local = DateTime.now();
      final utc = DateTime.now().toUtc();
      final difference =
          await api.howLongDoesItTake(mine: FeatureChrono(utc: utc, local: local, duration: duration, naive: naive));
      log('$difference');
    });
  });

  group('uuid feature', () {
    test('Uuid', () async {
      final uuid = Uuid();
      final id = uuid.v4obj();
      final output = await api.handleUuid(id: id);
      expect(id, output);
    });
    test('Vec<Uuid>', () async {
      final uuid = Uuid();
      final ids = List<UuidValue>.from([uuid.v4obj(), uuid.v1obj(), uuid.v4obj()]);
      final outputs = await api.handleUuids(ids: ids);
      expect(ids, outputs);
    });
    test('nested uuid types', () async {
      final uuid = Uuid();
      final id = uuid.v4obj();
      final ids = List<UuidValue>.from([uuid.v4obj(), uuid.v1obj(), uuid.v4obj()]);
      final wrapper = FeatureUuid(one: id, many: ids);
      final outputs = await api.handleNestedUuids(ids: wrapper);
      expect(wrapper.one, outputs.one);
      expect(wrapper.many, outputs.many);
    });
  });

  group('array feature', () {
    test('MessageId', () async {
      final MessageId msgid = await api.newMsgid(id: U8Array32.init());
      msgid.field0[2] = 14;
      final inner = await api.useMsgid(id: msgid);
      expect(inner[2], 14);
    });
    test('BlobId', () async {
      final inner = U8Array1600.init();
      inner[14] = 99;
      final Blob blob = await api.boxedBlob(blob: inner);
      expect(blob.field0[14], 99);
      blob.field0[10] = 100;
      final unboxed = await api.useBoxedBlob(blob: blob);
      expect(unboxed[10], 100);
      expect(unboxed[14], 99);
    });
    test('FeedId', () async {
      final inner = U8Array8.init();
      inner[3] = 3;
      final FeedId feedId = await api.returnBoxedFeedId(id: inner);
      expect(feedId.field0[3], 3);
      feedId.field0[5] = 5;
      final raw = await api.returnBoxedRawFeedId(id: feedId);
      expect(raw[5], 5);
      expect(raw[3], 3);
    });
    test('TestId', () async {
      final inner = I32Array2.init();
      inner[0] = 1;
      inner[1] = 2;
      final testId = await api.testId(id: TestId(field0: inner));
      expect(testId.field0[0], 1);
      expect(testId.field0[1], 2);
    });
    test('LastNumber', () async {
      final array = F64Array16.init();
      array[15] = 88.888;
      final lastNumber = await api.lastNumber(array: array);
      expect(lastNumber, 88.888);
    });
    test('NestedId', () async {
      final id0 = TestId(field0: I32Array2.init());
      id0.field0[1] = 10;
      final id1 = TestId(field0: I32Array2.init());
      id1.field0[1] = 20;
      final id2 = TestId(field0: I32Array2.init());
      id2.field0[1] = 30;
      final id3 = TestId(field0: I32Array2.init());
      id3.field0[1] = 40;
      final nestedId = await api.nestedId(id: TestIdArray4([id0, id1, id2, id3]));
      expect(nestedId[0].field0[1], 10);
      expect(nestedId[1].field0[1], 40);
    });
  });

  group('dart opaque type', () {
    String f() => 'Test_String';

    test('loopback', () async {
      await api.loopBackArrayGet(opaque: await api.loopBackArray(opaque: f));
      await api.loopBackVecGet(opaque: await api.loopBackVec(opaque: f));
      await api.loopBackOptionGet(opaque: await api.loopBackOption(opaque: f));

      var syncBack = api.syncLoopback(opaque: f);
      expect(identical(api.syncOptionLoopback(opaque: syncBack), f), isTrue);
      expect(api.syncOptionLoopback(opaque: null), isNull);

      var back1 = await api.loopBack(opaque: f) as String Function();
      expect(back1(), 'Test_String');
      var back2 = await api.loopBack(opaque: back1) as String Function();
      expect(back2(), 'Test_String');
      expect(identical(back2, f), isTrue);
    });

    test('drop', () async {
      expect(await api.asyncAcceptDartOpaque(opaque: createLargeList(mb: 200)), 'async test');
      expect(api.syncAcceptDartOpaque(opaque: createLargeList(mb: 200)), 'test');
    });

    test('unwrap', () async {
      expect(api.unwrapDartOpaque(opaque: createLargeList(mb: 200)), 'Test');
      await expectLater(
          () => api.panicUnwrapDartOpaque(opaque: createLargeList(mb: 200)), throwsA(isA<FfiException>()));
    });

    test('nested', () async {
      var str = await api.createNestedDartOpaque(opaque1: f, opaque2: f);
      await api.getNestedDartOpaque(opaque: str);
    });

    test('enum', () async {
      var en = await api.createEnumDartOpaque(opaque: f);
      await api.getEnumDartOpaque(opaque: en);
    });

    test('nested', () async {
      var str = await api.createNestedDartOpaque(opaque1: f, opaque2: f);
      await api.getNestedDartOpaque(opaque: str);
    });

    test('enum', () async {
      var en = await api.createEnumDartOpaque(opaque: f);
      await api.getEnumDartOpaque(opaque: en);
    });
  });

  group('rust opaque type', () {
    test('create and dispose', () async {
      var futureData = api.createOpaque();
      var data = await api.createOpaque();
      data.dispose();
      (await futureData).dispose();
    });

    test('simple call', () async {
      var opaque = await api.createOpaque();
      var hideData = await api.runOpaque(opaque: opaque);

      expect(
          hideData,
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      opaque.dispose();
    });

    test('double Call', () async {
      var data = await api.createOpaque();
      expect(
          await api.runOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(
          await api.runOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.dispose();
    });

    test('call after dispose', () async {
      var data = await api.createOpaque();
      expect(
          await api.runOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.dispose();
      await expectLater(() => api.runOpaque(opaque: data), throwsA(isA<FfiException>()));
    });

    test('dispose before complete', () async {
      var data = await api.createOpaque();
      var task = api.runOpaqueWithDelay(opaque: data);
      data.dispose();
      expect(
          await task,
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      await expectLater(() => api.runOpaque(opaque: data), throwsA(isA<FfiException>()));
    });

    test('create array of opaque type', () async {
      var data = await api.opaqueArray();
      for (var v in data) {
        expect(
            await api.runOpaque(opaque: v),
            "content - Some(PrivateData "
            "{"
            " content: \"content nested\", "
            "primitive: 424242, "
            "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
            "lifetime: \"static str\" "
            "})");
        v.dispose();
        await expectLater(() => api.runOpaque(opaque: v), throwsA(isA<FfiException>()));
      }
    });

    test('create enums of opaque type', () async {
      var data = await api.createArrayOpaqueEnum();

      expect(
          await api.runEnumOpaque(opaque: data[0]),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      (data[0] as EnumOpaque_Struct).field0.dispose();

      expect(await api.runEnumOpaque(opaque: data[1]), "42");
      (data[1] as EnumOpaque_Primitive).field0.dispose();

      expect(await api.runEnumOpaque(opaque: data[2]), "\"String\"");
      (data[2] as EnumOpaque_TraitObj).field0.dispose();

      expect(
          await api.runEnumOpaque(opaque: data[3]),
          "\"content - Some(PrivateData "
          "{"
          " content: \\\"content nested\\\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \\\"static str\\\" "
          "})\"");
      (data[3] as EnumOpaque_Mutex).field0.dispose();

      expect(
          await api.runEnumOpaque(opaque: data[4]),
          "\"content - Some(PrivateData "
          "{"
          " content: \\\"content nested\\\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \\\"static str\\\" "
          "})\"");
      (data[4] as EnumOpaque_RwLock).field0.dispose();
      await expectLater(() => api.runEnumOpaque(opaque: data[4]), throwsA(isA<FfiException>()));
    });

    test('opaque field', () async {
      var data = await api.createNestedOpaque();
      await api.runNestedOpaque(opaque: data);

      expect(
          await api.runOpaque(opaque: data.first),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(
          await api.runOpaque(opaque: data.second),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.first.dispose();
      await expectLater(() => api.runOpaque(opaque: data.first), throwsA(isA<FfiException>()));
      await expectLater(() => api.runNestedOpaque(opaque: data), throwsA(isA<FfiException>()));
      expect(
          await api.runOpaque(opaque: data.second),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.second.dispose();
    });

    test('array', () async {
      var data = await api.opaqueArray();
      await api.opaqueArrayRun(data: data);
      data[0].dispose();

      expect(
          await api.runOpaque(opaque: data[1]),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");

      await expectLater(() => api.opaqueArrayRun(data: data), throwsA(isA<FfiException>()));
      data[1].dispose();
    });

    test('vec', () async {
      var data = await api.opaqueVec();
      await api.opaqueVecRun(data: data);
      data[0].dispose();

      expect(
          await api.runOpaque(opaque: data[1]),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");

      await expectLater(() => api.opaqueVecRun(data: data), throwsA(isA<FfiException>()));
      data[1].dispose();
    });

    test('unwrap', () async {
      var data = await api.createOpaque();
      data.move = true;
      expect(
          await api.unwrapRustOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(data.isStale(), isTrue);

      var data2 = await api.createOpaque();
      await expectLater(() => api.unwrapRustOpaque(opaque: data2), throwsA(isA<FfiException>()));
      expect(data2.isStale(), isFalse);
    });
  });

  group('extended sync', () {
    test('check generator', () {
      expect(api.frbSyncGeneratorTest().runtimeType == FrbOpaqueSyncReturn, isTrue);
    });

    test('create', () {
      var data = api.syncCreateOpaque();
      data.dispose();
    });

    test('double call', () {
      var data = api.syncCreateSyncOpaque();
      expect(
          api.syncRunOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(
          api.syncRunOpaque(opaque: data),
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
      var data = api.syncCreateSyncOpaque();
      expect(
          api.syncRunOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.dispose();
      expect(() => api.syncRunOpaque(opaque: data), throwsA(isA<FfiException>()));
    });

    test('option', () async {
      var data = api.syncOption();
      var data2 = api.syncOptionNull();
      var data3 = api.syncOptionRustOpaque();
      var data4 = api.syncOptionDartOpaque(opaque: () => () => 'magic');
      expect(data, isNotNull);
      expect(data2, isNull);
      expect(data3, isNotNull);
      expect(data4, isNotNull);
      data3!.dispose();
    });

    test('void', () async {
      api.syncVoid();
    });

    test('unwrapped dart opaque', () async {
      String f() => "magic";
      var res = api.returnNonDroppableDartOpaque(opaque: f);
      expect(identical(res, f), isTrue);
    });

    test('dart call handle_type_id', () async {
      final id = await api.handleTypeAliasId(input: 42);
      expect(id, 42);
    });
    test('dart call handle_type_nest_alias_id', () async {
      final id = await api.handleTypeNestAliasId(input: 42);
      expect(id, 42);
    });
    test('dart call handle_type_model', () async {
      final testModel = await api.handleTypeAliasModel(input: 42);
      expect(testModel.id, 42);
      expect(testModel.name, "TestModel");
      expect(testModel.aliasEnum, MyEnum.False);
      expect(testModel.aliasStruct.content, true);
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
