import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:test/test.dart';

void main() {
  test('can mock Rust calls', () async {
    const mockResult = 123456789;
    WhatName.instance = theMockInstance;

    final actualResult = await simpleAdder(a: 1, b: 2);
    expect(actualResult, isNot(3));
    expect(actualResult, equals(mockResult));
  });
}
