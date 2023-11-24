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
}

// vim:expandtab:ts=2:sw=2
