import 'dart:async';

import 'package:meta/meta.dart';
import 'package:test/test.dart';

@isTestGroup
void addTestsIdentityFunctionCall<A, B>(
  FutureOr<A> Function({required B arg, dynamic hint}) func,
  List<dynamic> values,
) {
  group('call $func', () {
    for (final value in values) {
      test('with value $value', () async {
        expect(await func(arg: value), value);
      });
    }
  });
}
