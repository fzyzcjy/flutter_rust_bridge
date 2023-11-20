import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:test/test.dart';

void main() {
  // TODO p.s. use a mocking system that does not need code generation for simplicity
  // TODO or, `WhatName.init(instance: theMockInstance)`?
  WhatName.instance = theMockInstance;

  test('can mock Rust calls', () async {
    final actualResult = await simpleAdder(a: 1, b: 2);
    expect(actualResult, isNot(3));
    expect(actualResult, equals(123456789));
  });
}
