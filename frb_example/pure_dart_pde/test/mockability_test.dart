// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

// Surely, you can use Mockito or whatever other mocking packages
class MockRustLibApi extends Mock implements RustLibApi {}

Future<void> main() async {
  final mockApi = MockRustLibApi();
  when(() => mockApi.crateApiCustomizationInitApp())
      .thenAnswer((_) async => null);
  when(() => mockApi.crateApiCustomizationMyInitOne())
      .thenAnswer((_) async => null);
  when(() => mockApi.crateApiCustomizationMyInitTwo())
      .thenAnswer((_) async => null);

  await RustLib.init(api: mockApi);

  test('can mock Rust calls', () async {
    when(() => mockApi.crateApiSimpleSimpleAdderTwinNormal(a: 1, b: 2))
        .thenAnswer((_) async => 123456789);
    final actualResult = await simpleAdderTwinNormal(a: 1, b: 2);
    expect(actualResult, isNot(3));
    expect(actualResult, equals(123456789));
    verify(() => mockApi.crateApiSimpleSimpleAdderTwinNormal(a: 1, b: 2))
        .called(1);
  });
}
