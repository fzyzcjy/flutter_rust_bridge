import 'dart:async';
import 'dart:typed_data';

import 'package:meta/meta.dart';
import 'package:test/test.dart';

@isTestGroup
void addTestsIdentityFunctionCall<T>(
  FutureOr<T> Function({required T arg, dynamic hint}) func,
  List<T> values,
) {
  _addTestsRaw(
    groupName: 'call $func',
    values: values,
    body: (value) async => expect(await func(arg: value), value),
  );
}

@isTestGroup
void addTestsErrorFunctionCall<T>(
  FutureOr<void> Function({required T arg, dynamic hint}) func,
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

String? skipWeb([String reason = 'unspecified']) => kIsWeb ? 'Skipped on web (reason: $reason)' : null;

bool get releaseMode {
  var ans = true;
  assert(() {
    ans = false;
    return true;
  }());
  return ans;
}
