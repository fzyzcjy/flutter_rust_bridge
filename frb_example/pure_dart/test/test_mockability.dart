import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

// Surely, you can use Mockito or whatever other mocking packages
class MockWhatName extends Mock implements WhatName {}

void main() {
  // TODO p.s. use a mocking system that does not need code generation for simplicity
  // TODO or, `WhatName.init(instance: theMockInstance)`?
  final mockInstance = MockWhatName();
  WhatName.instance = mockInstance;

  test('can mock Rust calls', () async {
    when(() => mockInstance.simpleAdder()).thenReturn(123456789);
    final actualResult = await simpleAdder(a: 1, b: 2);
    expect(actualResult, isNot(3));
    expect(actualResult, equals(123456789));
    verify(() => mockInstance.simpleAdder()).called(1);
  });
}
