// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'dart:async';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:meta/meta.dart';
import 'package:test/test.dart';

@isTestGroup
void addTestsIdentityFunctionCall<T>(
  FutureOr<T> Function({required T arg}) func,
  List<T> values,
) {
  _addTestsRaw(
    groupName: 'call $func',
    values: values,
    body: (value) async => expect(await func(arg: value), value),
  );
}

@isTestGroup
void addTestsIdentityWithExpectFunctionCall<T>(
  FutureOr<T> Function({required T arg, required String expect}) func,
  List<T> values,
) {
  _addTestsRaw(
    groupName: 'call $func',
    values: values,
    body: (value) async =>
        expect(await func(arg: value, expect: value.toString()), value),
  );
}

@isTestGroup
void addTestsErrorFunctionCall<T>(
  FutureOr<void> Function({required T arg}) func,
  List<T> values,
  Matcher matcher(T value),
) {
  _addTestsRaw(
    groupName: 'call $func',
    values: values,
    body: (value) async => await expectLater(
      () async => func(arg: value),
      throwsA(matcher(value)),
    ),
  );
}

@isTestGroup
void _addTestsRaw<T>({
  required String groupName,
  required List<T> values,
  required Future<void> Function(T value) body,
}) {
  group(groupName, () {
    for (final value in values) {
      test('with value $value', () async {
        await body(value);
      });
    }
  });
}

void debugPrint(String message) {
  assert(() {
    print(message);
    return true;
  }());
}

Uint8List createLargeList({required int mb}) => Uint8List(1000000 * mb);

/// borrowed from flutter foundation [kIsWeb](https://api.flutter.dev/flutter/foundation/kIsWeb-constant.html),
/// but allows for using it in a Dart context alike
const bool kIsWeb = identical(0, 0.0);

String? skipWeb([String reason = 'unspecified']) =>
    kIsWeb ? 'Skipped on web (reason: $reason)' : null;

bool get releaseMode {
  var ans = true;
  assert(() {
    ans = false;
    return true;
  }());
  return ans;
}

Future<void> expectRustPanic(
  FutureOr<void> Function() body,
  String mode, {
  String? messageOnNative,
  Matcher? messageMatcherOnNative,
}) async {
  if (messageOnNative != null) {
    assert(messageMatcherOnNative == null);
    messageMatcherOnNative = startsWith(messageOnNative);
  }

  var inner = isA<PanicException>();
  if (!kIsWeb && messageMatcherOnNative != null) {
    inner = inner.having((x) => x.message, 'message', messageMatcherOnNative);
  }
  await expectRustPanicRaw(body, mode, throwsA(inner));
}

/// Temporary relax of the test due to limitation of Rust WASM.
///
/// 1. Only check message content on non-web, because of a current (2023.11) limitation of Rust WASM:
/// https://github.com/fzyzcjy/flutter_rust_bridge/blob/68b1a9415d5a47b9a793074974af8b2f8a8bcfc0/book/src/wasm_limitations.md
///
/// 2. When Web (WASM) + rust async mode + panic, expect the function call to hang,
/// again due to limitation of WASM before abort-unwind is usable.
///
/// But normal code should *not* rely on panic, so it should be OK.
Future<void> expectRustPanicRaw(
    FutureOr<void> Function() body, String mode, Matcher matcher) async {
  if (kIsWeb && mode.contains('RustAsync')) {
    print('expectRustPanicRaw check it should have no response');
    // expect it timeouts (hangs), instead of throws
    var bodyCompleted = false;
    unawaited(Future.value(body()).whenComplete(() => bodyCompleted = true));
    await Future.delayed(const Duration(milliseconds: 300));
    expect(bodyCompleted, false);
  } else {
    await expectLater(body, matcher);
  }
}

/// Hack to make generated pseudo-manual tests be happy about async and sync
Future<void> futurizeVoidTwinNormal(Future<void> x) async => x;

Future<void> futurizeVoidTwinRustAsync(Future<void> x) async => x;

Future<void> futurizeVoidTwinSync(void x) async => x;

Future<void> futurizeVoidTwinSse(Future<void> x) async => x;

Future<void> futurizeVoidTwinRustAsyncSse(Future<void> x) async => x;

Future<void> futurizeVoidTwinSyncSse(void x) async => x;

Future<void> futurizeVoidTwinMoi(Future<void> x) async => x;

Future<void> futurizeVoidTwinRustAsyncMoi(Future<void> x) async => x;

Future<void> futurizeVoidTwinSyncMoi(void x) async => x;

Future<void> futurizeVoidTwinSseMoi(Future<void> x) async => x;

Future<void> futurizeVoidTwinRustAsyncSseMoi(Future<void> x) async => x;

Future<void> futurizeVoidTwinSyncSseMoi(void x) async => x;
