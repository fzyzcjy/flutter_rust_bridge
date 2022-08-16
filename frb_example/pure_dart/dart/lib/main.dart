import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';
import 'ffi.dart' if (dart.library.html) 'ffi.web.dart';
import 'bridge_definitions.dart';

void main(List<String> args) async {
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final api = initializeExternalLibrary(dylibPath);

  test('dart call simpleAdder', () async {
    expect(await api.simpleAdder(a: 42, b: 100), 142);
  });

  test('dart call primitiveTypes', () async {
    expect(
        await api.primitiveTypes(myI32: 123, myI64: 10000000000000, myF64: 12345678901234567890.123, myBool: true), 42);
  });

  test('dart call primitiveU32', () async {
    expect(await api.primitiveU32(myU32: 0xff112233), 0xfe112233);
  });

  test('dart call handleReturnUnit', () async {
    await api.handleReturnUnit();
  });

  test('dart call handleString', () async {
    expect(await api.handleString(s: "Hello, world!"), "Hello, world!Hello, world!");
  });

  test('dart call handleVecU8', () async {
    final len = 100000;
    expect(await api.handleVecU8(v: Uint8List.fromList(List.filled(len, 127))),
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

  test('dart call handleStruct', () async {
    final structResp =
        await api.handleStruct(arg: MySize(width: 42, height: 100), boxed: MySize(width: 1000, height: 10000));
    expect(structResp.width, 42 + 1000);
    expect(structResp.height, 100 + 10000);
  });

  test('dart call handleNewtype', () async {
    final newtypeResp = await api.handleNewtype(arg: NewTypeInt(field0: 42));
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

  test('dart call handleStringList', () async {
    final names = await api.handleStringList(names: ['Steve', 'Bob', 'Alex']);
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

  Future<void> _testHandleStream(
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
    _testHandleStream(api.handleStreamSinkAt1);
  });

  test('dart call handle_stream_sink_at_2', () {
    _testHandleStream(api.handleStreamSinkAt2);
  });

  test('dart call handle_stream_sink_at_3', () {
    _testHandleStream(api.handleStreamSinkAt3);
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
    expect(await api.getArray(), [1, 2, 3, 4, 5]);
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

  test('ConcatenateWith stream sink test', () async {
    final ConcatenateWith concatenateWith = ConcatenateWith(a: "hello ", bridge: api);
    final int key = 10;
    final int max = 5;
    final stream = concatenateWith.handleSomeStreamSink(key: key, max: max);
    int cnt = 0;
    print('start listening');
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
    print('start listening');
    await for (final value in stream) {
      print("output from ConcatenateWith's static stream: $value");
      expect(value.value, "$cnt");
      cnt++;
    }
    expect(cnt, max);
  });

  test('ConcatenateWith static stream sink at 1 test', () async {
    final stream = ConcatenateWith.handleSomeStaticStreamSinkSingleArg(bridge: api);
    print('start listening');
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
    expect((Speed_Unknown).toString(), 'Speed_Unknown');
    expect((Speed_GPS).toString(), 'Speed_GPS');
    expect((Distance_Unknown).toString(), 'Distance_Unknown');
    expect((Distance_Map).toString(), 'Distance_Map');
  });

  group('Platform-specific support', () {
    test('Int64List', () {
      final list = Int64List.fromList([-1, -2, -3, -4, -5]);
      expect(list[0].toString(), '-1');
      expect(list.map((el) => el * el), DeepMatchesLooseInts([1, 4, 9, 16, 25]));
      list[1] = -123;
      expect(list[1].toString(), '-123');
    });
    test('Uint64List', () {
      final list = Uint64List.fromList([1, 2, 3, 4, 5]);
      expect(list[0].toString(), '1');
      expect(list.map((el) => el * el), DeepMatchesLooseInts([1, 4, 9, 16, 25]));
      list[1] = 123;
      expect(list[1].toString(), '123');
    });
  });

  print('flutter_rust_bridge example program end');
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

class DeepMatchesLooseInts extends CustomMatcher {
  /// On WASM platforms, indexing into a [Int64List] or [Uint64List] will return
  /// an Int64 from the fixnum library, which fails the type test. This asserts
  /// that they only have to match by their values.
  DeepMatchesLooseInts(matcher) : super("is a numeric", "value", matcher);
  @override
  Object? featureValueOf(actual) {
    if (actual is Iterable) return actual.map(featureValueOf);
    if (actual is int) return actual;
    return int.tryParse('$actual');
  }
}

// vim:expandtab:ts=2:sw=2
