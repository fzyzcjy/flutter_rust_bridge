import 'dart:async';

import 'package:meta/meta.dart';
import 'package:test/test.dart';

@isTestGroup
void addTestsIdentityFunctionCall<T>(
  FutureOr<T> Function({required T arg, dynamic hint}) func,
  List<T> values,
) {
  group('call $func', () {
    for (final value in values) {
      test('with value $value', () async {
        expect(await func(arg: value), value);
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
