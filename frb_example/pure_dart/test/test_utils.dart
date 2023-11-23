import 'dart:async';

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
