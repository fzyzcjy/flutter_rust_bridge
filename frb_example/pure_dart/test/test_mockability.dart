import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

// Surely, you can use Mockito or whatever other mocking packages
class MockFrbGeneratedPureDart extends Mock implements FrbGeneratedPureDart {}

Future<void> main() async {
  final mockInstance = MockFrbGeneratedPureDart();
  await FrbGeneratedPureDart.init(mockInstance);

  test('can mock Rust calls', () async {
    when(() => mockInstance.simpleAdder()).thenReturn(123456789);
    final actualResult = await simpleAdder(a: 1, b: 2);
    expect(actualResult, isNot(3));
    expect(actualResult, equals(123456789));
    verify(() => mockInstance.simpleAdder()).called(1);
  });
}
