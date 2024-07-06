// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_fn_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/dart_fn_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('rustCallDartSimpleTwinSse', () async {
    var callbackCallCount = 0;
    await rustCallDartSimpleTwinSse(callback: () => callbackCallCount++);
    expect(callbackCallCount, 1);
  });

  test('rustCallDartOneArgTwinSse', () async {
    final callbackCalls = <String>[];
    await rustCallDartOneArgTwinSse(callback: (arg) => callbackCalls.add(arg));
    expect(callbackCalls, ['a']);
  });

  test('rustCallDartTwoArgsTwinSse', () async {
    final callbackCalls = <(String, DemoStructForRustCallDartTwinSse)>[];
    await rustCallDartTwoArgsTwinSse(
        callback: (a, b) => callbackCalls.add((a, b)));
    expect(callbackCalls, [('a', DemoStructForRustCallDartTwinSse(name: 'b'))]);
  });

  test('rustCallDartReturnTwinSse', () async {
    var callCount = 0;
    await rustCallDartReturnTwinSse(callback: () {
      callCount++;
      return 'a';
    });
    expect(callCount, 1);
  });

  test('rustCallDartLoopbackTwinSse', () async {
    var callCount = 0;
    await rustCallDartLoopbackTwinSse(callback: (arg) {
      callCount++;
      return arg;
    });
    expect(callCount, 1);
  });

  test('rust closure be asynchronous', () async {
    var callCount = 0;
    await rustCallDartLoopbackTwinSse(callback: (arg) async {
      callCount++;
      return arg;
    });
    expect(callCount, 1);
  });

  test('rustCallDartWithDartOpaqueArgTwinSse', () async {
    final opaque = (String whatever) => 42;
    var callbackCalls = <dynamic>[];
    await rustCallDartWithDartOpaqueArgTwinSse(
        input: opaque, callback: (arg) => callbackCalls.add(arg));
    expect(callbackCalls[0]('hello'), 42);
  });

  test('rustCallDartWithDartOpaqueResultTwinSse', () async {
    final opaque = (String whatever) => 42;
    var callCount = 0;
    final dynamic output =
        await rustCallDartWithDartOpaqueResultTwinSse(callback: () {
      callCount++;
      return opaque;
    });
    expect(callCount, 1);
    expect(output('hello'), 42);
  });

  test('rustCallDartMultiTimesTwinSse', () async {
    var callCount = 0;
    await rustCallDartMultiTimesTwinSse(
        callback: () => callCount++, numTimes: 10);
    expect(callCount, 10);
  });

  group('rustCallDartReturnResultTwinSse', () {
    test('when normal', () async {
      await rustCallDartReturnResultTwinSse(
          callback: (s) => s * 2, expectOutput: "hihi");
    });

    test('when error', () async {
      await rustCallDartReturnResultTwinSse(
          callback: (s) => throw Exception('dummy exception'),
          expectOutput: null);
    });
  });
}
