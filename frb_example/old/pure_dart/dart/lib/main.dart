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

  test('dart call app_settings_stream', () async {
    final settings = await appSettingsStream().first;
    testAppSettings(settings);
  });

  test('dart call app_settings_vec_stream', () async {
    final settings = await appSettingsVecStream().first;
    testAppSettings(settings[0]);
    testAppSettings(settings[1]);
  });

  test('dart call mirror_struct_stream', () async {
    final ret = await mirrorStructStream().first;
    testAppSettings(ret.a);
    expect(ret.b.content, true);
    expect(ret.c[0], MyEnum.True);
    expect(ret.c[1], MyEnum.False);
    testAppSettings(ret.d[0]);
    testAppSettings(ret.d[1]);
  });

  test('dart call mirror_tuple_stream', () async {
    final (settings, rawStringEnum) = await mirrorTupleStream().first;
    testAppSettings(settings);
    expect(rawStringEnum is RawStringEnumMirrored_Raw, true);
    expect((rawStringEnum as RawStringEnumMirrored_Raw).field0.value, "test");
  });

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

  test('dart call getAppSettings()', () async {
    var settings = await getAppSettings();
    expect(settings.version, "1.0.0-rc.1");
    expect(settings.mode, ApplicationMode.standalone);
    expect(settings.env.vars[0].field0, "myenv");
  });

  test('dart call isAppEmbedded()', () async {
    expect(
        await isAppEmbedded(
            appSettings: ApplicationSettings(
                name: "from dart",
                version: "XX",
                mode: ApplicationMode.embedded,
                env: ApplicationEnv(vars: [ApplicationEnvVar(field0: "sendback", field1: true)]))),
        true);
  });

  test('dart call getMessage()', () async {
    var message = await getMessage();
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
    var numbers = await repeatNumber(num: 1, times: 10);
    expect(numbers.field0.toList(), Int32List.fromList([1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
  });

  test('dart call repeatSequence()', () async {
    var sequences = await repeatSequence(seq: 1, times: 10);
    expect(sequences.field0.toList(), Int32List.fromList([1, 1, 1, 1, 1, 1, 1, 1, 1, 1]));
  });

  test('dart call firstNumber()', () async {
    var numbers = Numbers(field0: Int32List.fromList([1]));
    var first = await firstNumber(nums: numbers);
    expect(first, 1);
  });

  test('dart call firstSequence()', () async {
    var sequences = Sequences(field0: Int32List.fromList([1]));
    var first = await firstSequence(seqs: sequences);
    expect(first, 1);
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

  test('dart check that non-final field is modifiable', () {
    var customized = Customized(finalField: "finalField", nonFinalField: "nonFinalField");
    expect(customized.nonFinalField, "nonFinalField");
    customized.nonFinalField = "changed";
    expect(customized.nonFinalField, "changed");
  });

  test('dart call next_user_id to test metadata annotations', () async {
    UserId userId = UserId(value: 11);
    expect(await nextUserId(userId: userId), UserId(value: 12));
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

  test('test mirrored raw structs', () async {
    final output = await testRawStringMirrored();
    expect(output, isA<RawStringMirrored>());
    expect(output.value, "test");
  });

  test('test nested mirror raw', () async {
    final output = await testNestedRawStringMirrored();
    expect(output, isA<NestedRawStringMirrored>());
    expect(output.raw, isA<RawStringMirrored>());
    expect(output.raw.value, "test");
  });

  test('test raw string enum', () async {
    final output1 = await testRawStringEnumMirrored(nested: true);
    expect(output1 is RawStringEnumMirrored_Nested, true);
    expect((output1 as RawStringEnumMirrored_Nested).field0.raw.value, "test");

    final output2 = await testRawStringEnumMirrored(nested: false);
    expect(output2 is RawStringEnumMirrored_Raw, true);
    expect((output2 as RawStringEnumMirrored_Raw).field0.value, "test");
  });

  test('test list of raw nested strings', () async {
    final output = await testListOfRawNestedStringMirrored();
    expect(output.raw.length, 1);
    expect(output.raw[0].raw.value, "test");
  });

  test('test fallible vec of raw string', () async {
    final output = await testFallibleOfRawStringMirrored();
    expect(output.length, 1);
    expect(output.first.value, "test");
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

  test('test contains mirrored sub struct', () async {
    final output = await testContainsMirroredSubStruct();
    expect(output, isA<ContainsMirroredSubStruct>());
    expect(output.test, isA<RawStringMirrored>());
    expect(output.test.value, "test");
    expect(output.test2.a, "test");
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

  group('chrono feature', () {
    test('DateTime<Utc>', () async {
      final date = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
      final resp = await datetimeUtc(d: date);
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
      final resp = await datetimeLocal(d: date);
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
      final resp = await naivedatetime(d: date);
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
      final resp = await optionalEmptyDatetimeUtc(d: null);
      expect(resp, null);
    });

    test('Duration', () async {
      final duration = Duration(hours: 4);
      final resp = await duration(d: duration);
      expect(resp.inHours, duration.inHours);
    });

    test('List<Duration>', () {
      final expected = [
        Duration(days: 1),
        Duration(days: 10),
        Duration(days: 100),
        Duration(milliseconds: 333),
        if (!kIsWeb) Duration(microseconds: 333)
      ];
      final now = DateTime.now();
      final durations = handleTimestamps(
        timestamps: expected.map(now.subtract).toList(),
        epoch: now,
      );
      expect(durations, completion(expected));
    });

    test('List<DateTime>', () {
      final expected = [
        Duration(days: 3),
        Duration(hours: 2),
        Duration(seconds: 1),
        Duration(milliseconds: 500),
        if (!kIsWeb) Duration(microseconds: 400)
      ];
      final now = DateTime.now();
      final result = handleDurations(durations: expected, since: now);
      expect(result, completion(expected.map(now.subtract)));
    });

    test('Combined Chrono types', () async {
      final test = await testChrono();
      expect(castInt(test.dt!.millisecondsSinceEpoch), castInt(1631297333000));
      expect(castInt(test.dt2!.millisecondsSinceEpoch), castInt(1631297333000));
      expect(test.du, Duration(hours: 4));
    });

    test('combined chrono types precise', () async {
      final datetime_1 = DateTime.utc(2002, 02, 23, 12, 13, 55);
      final datetime_2 = DateTime.utc(1800, 01, 23, 12, 56, 25);
      final duration = Duration(hours: 4);

      final result = await testPreciseChrono();

      expect(result.dt!.millisecondsSinceEpoch, datetime_1.millisecondsSinceEpoch);
      expect(result.dt2!.millisecondsSinceEpoch, datetime_2.millisecondsSinceEpoch);
      expect(result.du!.inHours, duration.inHours);
    });

    test('nested chrono types', () async {
      const duration = Duration(hours: 4);
      final naive = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
      final local = DateTime.now();
      final utc = DateTime.now().toUtc();
      final difference =
          await howLongDoesItTake(mine: FeatureChrono(utc: utc, local: local, duration: duration, naive: naive));
      log('$difference');
    });
  });

  group('uuid feature', () {
    test('Uuid', () async {
      final uuid = Uuid();
      final id = uuid.v4obj();
      final output = await handleUuid(id: id);
      expect(id, output);
    });
    test('Vec<Uuid>', () async {
      final uuid = Uuid();
      final ids = List<UuidValue>.from([uuid.v4obj(), uuid.v1obj(), uuid.v4obj()]);
      final outputs = await handleUuids(ids: ids);
      expect(ids, outputs);
    });
    test('nested uuid types', () async {
      final uuid = Uuid();
      final id = uuid.v4obj();
      final ids = List<UuidValue>.from([uuid.v4obj(), uuid.v1obj(), uuid.v4obj()]);
      final wrapper = FeatureUuid(one: id, many: ids);
      final outputs = await handleNestedUuids(ids: wrapper);
      expect(wrapper.one, outputs.one);
      expect(wrapper.many, outputs.many);
    });
  });

  group('dart opaque type', () {
    String f() => 'Test_String';

    test('loopback', () async {
      await loopBackArrayGet(opaque: await loopBackArray(opaque: f));
      await loopBackVecGet(opaque: await loopBackVec(opaque: f));
      await loopBackOptionGet(opaque: await loopBackOption(opaque: f));

      var syncBack = syncLoopback(opaque: f);
      expect(identical(syncOptionLoopback(opaque: syncBack), f), isTrue);
      expect(syncOptionLoopback(opaque: null), isNull);

      var back1 = await loopBack(opaque: f) as String Function();
      expect(back1(), 'Test_String');
      var back2 = await loopBack(opaque: back1) as String Function();
      expect(back2(), 'Test_String');
      expect(identical(back2, f), isTrue);
    });

    test('drop', () async {
      expect(await asyncAcceptDartOpaque(opaque: createLargeList(mb: 200)), 'async test');
      expect(syncAcceptDartOpaque(opaque: createLargeList(mb: 200)), 'test');
    });

    test('unwrap', () async {
      expect(unwrapDartOpaque(opaque: createLargeList(mb: 200)), 'Test');
      await expectLater(() => panicUnwrapDartOpaque(opaque: createLargeList(mb: 200)), throwsA(isA<PanicException>()));
    });

    test('nested', () async {
      var str = await createNestedDartOpaque(opaque1: f, opaque2: f);
      await getNestedDartOpaque(opaque: str);
    });

    test('enum', () async {
      var en = await createEnumDartOpaque(opaque: f);
      await getEnumDartOpaque(opaque: en);
    });

    test('nested', () async {
      var str = await createNestedDartOpaque(opaque1: f, opaque2: f);
      await getNestedDartOpaque(opaque: str);
    });

    test('enum', () async {
      var en = await createEnumDartOpaque(opaque: f);
      await getEnumDartOpaque(opaque: en);
    });
  });

  group('rust opaque type', () {
    test('create and dispose', () async {
      var futureData = createOpaque();
      var data = await createOpaque();
      data.dispose();
      (await futureData).dispose();
    });

    test('simple call', () async {
      var opaque = await createOpaque();
      var hideData = await runOpaque(opaque: opaque);

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
      var data = await createOpaque();
      expect(
          await runOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(
          await runOpaque(opaque: data),
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
      var data = await createOpaque();
      expect(
          await runOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.dispose();
      await expectLater(() => runOpaque(opaque: data), throwsA(isA<PanicException>()));
    });

    test('dispose before complete', () async {
      var data = await createOpaque();
      var task = runOpaqueWithDelay(opaque: data);
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
      await expectLater(() => runOpaque(opaque: data), throwsA(isA<PanicException>()));
    });

    test('create array of opaque type', () async {
      var data = await opaqueArray();
      for (var v in data) {
        expect(
            await runOpaque(opaque: v),
            "content - Some(PrivateData "
            "{"
            " content: \"content nested\", "
            "primitive: 424242, "
            "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
            "lifetime: \"static str\" "
            "})");
        v.dispose();
        await expectLater(() => runOpaque(opaque: v), throwsA(isA<PanicException>()));
      }
    });

    test('create enums of opaque type', () async {
      var data = await createArrayOpaqueEnum();

      expect(
          await runEnumOpaque(opaque: data[0]),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      (data[0] as EnumOpaque_Struct).field0.dispose();

      expect(await runEnumOpaque(opaque: data[1]), "42");
      (data[1] as EnumOpaque_Primitive).field0.dispose();

      expect(await runEnumOpaque(opaque: data[2]), "\"String\"");
      (data[2] as EnumOpaque_TraitObj).field0.dispose();

      expect(
          await runEnumOpaque(opaque: data[3]),
          "\"content - Some(PrivateData "
          "{"
          " content: \\\"content nested\\\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \\\"static str\\\" "
          "})\"");
      (data[3] as EnumOpaque_Mutex).field0.dispose();

      expect(
          await runEnumOpaque(opaque: data[4]),
          "\"content - Some(PrivateData "
          "{"
          " content: \\\"content nested\\\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \\\"static str\\\" "
          "})\"");
      (data[4] as EnumOpaque_RwLock).field0.dispose();
      await expectLater(() => runEnumOpaque(opaque: data[4]), throwsA(isA<PanicException>()));
    });

    test('opaque field', () async {
      var data = await createNestedOpaque();
      await runNestedOpaque(opaque: data);

      expect(
          await runOpaque(opaque: data.first),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(
          await runOpaque(opaque: data.second),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      data.first.dispose();
      await expectLater(() => runOpaque(opaque: data.first), throwsA(isA<PanicException>()));
      await expectLater(() => runNestedOpaque(opaque: data), throwsA(isA<PanicException>()));
      expect(
          await runOpaque(opaque: data.second),
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
      var data = await opaqueArray();
      await opaqueArrayRun(data: data);
      data[0].dispose();

      expect(
          await runOpaque(opaque: data[1]),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");

      await expectLater(() => opaqueArrayRun(data: data), throwsA(isA<PanicException>()));
      data[1].dispose();
    });

    test('vec', () async {
      var data = await opaqueVec();
      await opaqueVecRun(data: data);
      data[0].dispose();

      expect(
          await runOpaque(opaque: data[1]),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");

      await expectLater(() => opaqueVecRun(data: data), throwsA(isA<PanicException>()));
      data[1].dispose();
    });

    test('unwrap', () async {
      var data = await createOpaque();
      data.move = true;
      expect(
          await unwrapRustOpaque(opaque: data),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      expect(data.isStale(), isTrue);

      var data2 = await createOpaque();
      await expectLater(() => unwrapRustOpaque(opaque: data2), throwsA(isA<FrbAnyhowException>()));
      expect(data2.isStale(), isFalse);
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

    test('dart call handle_type_id', () async {
      final id = await handleTypeAliasId(input: 42);
      expect(id, 42);
    });
    test('dart call handle_type_nest_alias_id', () async {
      final id = await handleTypeNestAliasId(input: 42);
      expect(id, 42);
    });
    test('dart call handle_type_model', () async {
      final testModel = await handleTypeAliasModel(input: 42);
      expect(testModel.id, 42);
      expect(testModel.name, "TestModel");
      expect(testModel.aliasEnum, MyEnum.False);
      expect(testModel.aliasStruct.content, true);
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
