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

  test('loop and call many times', () async {
    var obj = _createMyTreeNode(arrLen: 5);
    for (var i = 0; i < 500; ++i) {
      obj = await handleComplexStruct(s: obj);
    }
  });

  test('dart call getUsize', () async {
    expect(await getUsize(u: 2), 2);
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

  test('test empty struct', () async {
    final empty = Empty();
    final output = await emptyStruct(empty: empty);
    expect(output, isA<Empty>());
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
  });

  group('extended sync', () {
    test('create', () {
      var data = syncCreateOpaque();
      data.dispose();
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

// vim:expandtab:ts=2:sw=2
