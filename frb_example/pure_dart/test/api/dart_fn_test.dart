// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "syncSse"]}

import 'package:frb_example_pure_dart/src/rust/api/dart_fn.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('rustCallDartSimpleTwinNormal', () async {
    var callbackCallCount = 0;
    await rustCallDartSimpleTwinNormal(callback: () => callbackCallCount++);
    expect(callbackCallCount, 1);
  });

  test('rustCallDartOneArgTwinNormal', () async {
    final callbackCalls = <String>[];
    await rustCallDartOneArgTwinNormal(
        callback: (arg) => callbackCalls.add(arg));
    expect(callbackCalls, ['a']);
  });

  test('rustCallDartTwoArgsTwinNormal', () async {
    final callbackCalls = <(String, DemoStructForRustCallDartTwinNormal)>[];
    await rustCallDartTwoArgsTwinNormal(
        callback: (a, b) => callbackCalls.add((a, b)));
    expect(
        callbackCalls, [('a'), DemoStructForRustCallDartTwinNormal(name: 'b')]);
  });

  test('rustCallDartReturnTwinNormal', () async {
    var callCount = 0;
    await rustCallDartReturnTwinNormal(callback: () {
      callCount++;
      return 'a';
    });
    expect(callCount, 1);
  });

  test('rustCallDartLoopbackTwinNormal', () async {
    var callCount = 0;
    await rustCallDartLoopbackTwinNormal(callback: (arg) {
      callCount++;
      return arg;
    });
    expect(callCount, 1);
  });

  test('rust closure be asynchronous', () async {
    var callCount = 0;
    await rustCallDartLoopbackTwinNormal(callback: (arg) async {
      callCount++;
      return arg;
    });
    expect(callCount, 1);
  });

  test('rustCallDartWithDartOpaqueArgTwinNormal', () async {
    final opaque = (String whatever) => 42;
    var callCount = 0;
    await rustCallDartWithDartOpaqueArgTwinNormal(
      input: opaque,
      callback: (arg) {
        callCount++;
        expect(arg, opaque);
      },
    );
    expect(callCount, 1);
  });

  test('rustCallDartWithDartOpaqueResultTwinNormal', () async {
    final opaque = (String whatever) => 42;
    var callCount = 0;
    final output =
        await rustCallDartWithDartOpaqueResultTwinNormal(callback: () {
      callCount++;
      return opaque;
    });
    expect(callCount, 1);
    expect(output, opaque);
  });

  test('rustCallDartMultiTimesTwinNormal', () async {
    var callCount = 0;
    await rustCallDartMultiTimesTwinNormal(
        callback: () => callCount++, numTimes: 10);
    expect(callCount, 10);
  });
}
